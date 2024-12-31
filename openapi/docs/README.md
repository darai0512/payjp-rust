# commands

root.jsonnet をビルドして openapi.json を生成する

```
$ docker compose up -d

# if error, check logs
$ docker compose logs -f

# rm all ps
$ docker rm $(docker stop $(docker ps -aq -f name=payjp-rust-*))
```

- jsonnet 内で$refのパス解決のためのrenameをしているが、処理が遅い。簡潔さのためjsonnetを選んだがNode.jsが良さそう
  - gen (=openapi-generator-cli) はvalidation目的で実行しているが、$refのパス解決にも使えなくはない。

※ foreground実行するとshellにコントロールが戻ってこない？ような事象が発生する。未解決。
cf. https://github.com/docker/compose/issues/3347