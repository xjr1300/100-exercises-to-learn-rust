# A dedicated `Client` type（専用のClient型）

All the interactions from the client side have been fairly low-level: you have to
manually create a response channel, build the command, send it to the server, and
then call `recv` on the response channel to get the response.

> クライアント側からのすべての相互作用は、かなり低水準でした。
> 手動でレスポンス用のチャネルを作成、コマンドを構築、それをサーバーに送信、レスポンスを得るためにレスポンス用のチャネルに`recv`を呼び出さなくてはなりません。

This is a lot of boilerplate code that could be abstracted away, and that's
exactly what we're going to do in this exercise.

> これは、抽象化できる多くの定型コードで、それは正確にこの演習で行うことです。
