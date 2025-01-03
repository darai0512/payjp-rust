use miniserde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use payjp_types::{AsCursorOpt, List, Object, };

use crate::{RequestBuilder, BlockingClient, PayjpClient, PayjpMethod};

/// A trait allowing `List<T>` to be treated the same. Not part of the
/// public API.
///
/// NB: this trait is designed specifically for `List` and may not be sensible
/// in other cases. One gotcha is that `into_parts` and `from_parts` do not necessarily
/// round-trip, e.g. `List<T>` will lose the `next_page` field since that
/// is not part of the shared list impl. We account for this by ensuring to call `update_params`
/// before breaking the `List` into pieces.
#[doc(hidden)]
pub trait PaginableList: Deserialize {
    /// Underlying single element type, e.g. `Account`
    type Data;

    /// Break into the shared parts list pagination requires
    fn into_parts(self) -> ListParts<Self::Data>;

    /// Reconstruct from the shared parts list pagination requires
    fn from_parts(parts: ListParts<Self::Data>) -> Self;

    /// Update the current parameter set, with `self` as the most
    /// recently fetched page.
    ///
    /// NB: this should also set `has_more` to `false` explicitly if we don't have a new cursor.
    /// (This seems redundant with `has_more` but is used to provide extra protection
    /// against any possibility where `has_more` is `true`, but the cursor is back to `None`,
    /// potentially leading to an infinite pagination loop).
    fn update_params(&mut self, params: &mut Value);
}

/// Specific list parts relied on by the client for pagination.
#[doc(hidden)]
#[derive(Debug)]
pub struct ListParts<T> {
    pub count: Option<u64>,
    pub url: String,
    pub data: Vec<T>,
    pub has_more: bool,
}

impl<T> PaginableList for List<T>
where
    T: Object,
    List<T>: Deserialize,
{
    type Data = T;

    fn into_parts(self) -> ListParts<Self::Data> {
        ListParts {
            count: self.count,
            url: self.url,
            data: self.data,
            has_more: self.has_more,
        }
    }

    fn from_parts(parts: ListParts<Self::Data>) -> Self {
        Self {
            data: parts.data,
            has_more: parts.has_more,
            count: parts.count,
            url: parts.url,
        }
    }

    fn update_params(&mut self, params: &mut Value) {
        if let Some(new_cursor) = self.data.last().and_then(|l| l.id().as_cursor_opt()) {
            params["starting_after"] = Value::String(new_cursor.into());
        } else {
            self.has_more = false;
        }
    }
}

/// An extension trait to allow converting `List<T>` into
/// a type that can be paginated. Not meant to be implemented by any other types.
pub trait PaginationExt {
    /// The underlying pagination type, e.g. `List<T>`
    type Data;

    /// Use the current page state to construct an adaptor capable of paginating
    /// from where the current data left off.
    fn into_paginator(self) -> ListPaginator<Self::Data>;
}

impl<T> PaginationExt for List<T>
where
    T: Sync + Send + 'static,
    List<T>: PaginableList,
{
    type Data = List<T>;

    fn into_paginator(mut self) -> ListPaginator<List<T>> {
        let mut params = Default::default();
        self.update_params(&mut params);
        ListPaginator { page: self, params }
    }
}

/// Stream designed to support pagination.
#[derive(Debug)]
pub struct ListPaginator<T> {
    page: T,
    params: Value,
}

impl<T> ListPaginator<List<T>> {
    /// Kept public so that the generated code crates can access this trait. Used by `List*` params
    /// to implement `PaginationExt`. Not part of the public API.
    #[doc(hidden)]
    pub fn new_list(url: impl Into<String>, params: impl Serialize) -> Self {
        let page = List { data: vec![], has_more: true, count: None, url: url.into() };
        Self {
            page,
            params: serde_json::to_value(params)
                .expect("all types implement `Serialize` infallibly"),
        }
    }
}

fn req_builder(url: &str) -> RequestBuilder {
    RequestBuilder::new(PayjpMethod::Get, url.trim_start_matches("/v1"))
}

impl<T> ListPaginator<T>
where
    T: Sync + Send + 'static + PaginableList,
{
    /// Repeatedly queries for more data until all elements in list are fetched, using
    ///
    /// # Errors
    /// If any pagination request returns an error.
    pub fn get_all<C: BlockingClient>(self, client: &C) -> Result<Vec<T::Data>, C::Err> {
        let mut data = vec![];
        let mut parts = self.page.into_parts();
        let mut params = self.params;
        loop {
            // `append` empties `parts.data`
            data.append(&mut parts.data);

            if !parts.has_more {
                break;
            }

            let req = req_builder(&parts.url).query(&params);
            let mut next_page: T = req.customize().send_blocking(client)?;
            next_page.update_params(&mut params);
            parts = next_page.into_parts();
        }
        Ok(data)
    }

    /// Get all values in this List, consuming self and lazily paginating until all values are fetched.
    ///
    /// This function repeatedly queries for more data until all elements in list are fetched.
    pub fn stream<C: PayjpClient + Clone>(
        self,
        client: &C,
    ) -> impl futures_util::Stream<Item = Result<T::Data, C::Err>> + Unpin {
        // We are going to be popping items off the end of the list, so we need to reverse it.
        let mut page = self.page.into_parts();
        page.data.reverse();
        let paginator = ListPaginator { page: T::from_parts(page), params: self.params };

        Box::pin(futures_util::stream::unfold(
            Some((paginator, client.clone())),
            Self::unfold_stream,
        ))
    }

    /// Unfold a single item from the stream.
    async fn unfold_stream<C: PayjpClient + Clone>(
        state: Option<(Self, C)>,
    ) -> Option<(Result<T::Data, C::Err>, Option<(Self, C)>)> {
        let (paginator, client) = state?; // If none, our last request was an error, so stop here
        let mut parts = paginator.page.into_parts();
        if let Some(next_val) = parts.data.pop() {
            // We have more data on this page
            return Some((
                Ok(next_val),
                Some((Self { page: T::from_parts(parts), params: paginator.params }, client)),
            ));
        }

        // Final value of the stream, no errors
        if !parts.has_more {
            return None;
        }

        let req = req_builder(&parts.url).query(&paginator.params);
        match req.customize::<T>().send(&client).await {
            Ok(mut next_page) => {
                let mut params = paginator.params;
                next_page.update_params(&mut params);

                let mut parts = next_page.into_parts();

                // We are going to be popping items off the end of the list, so we need to reverse it.
                parts.data.reverse();

                let next_val = parts.data.pop()?;

                // Yield last value of this page, the next page (and client) becomes the state
                Some((Ok(next_val), Some((Self { page: T::from_parts(parts), params }, client))))
            }
            Err(err) => Some((Err(err), None)), // We ran into an error. The last value of the stream will be the error.
        }
    }
}
