# A dedicated `Client` type（専用のClient型）

All the interactions from the client side have been fairly low-level: you have to
manually create a response channel, build the command, send it to the server, and
then call `recv` on the response channel to get the response.

> クライアント側からのすべての相互作用は、かなり低水準でした。
> 手動でレスポンス用のチャネルを作成して、コマンドを構築、それ（レスポンス用のチャネル）をサーバーに送信、そしてレスポンスを得るためにレスポンス用のチャネルに`recv`を呼び出さなくてはなりません。

This is a lot of boilerplate code that could be abstracted away, and that's
exactly what we're going to do in this exercise.

> これは、抽象化できる多くの定型コードで、さにそれがこの演習で行うことです。

> 通常の通信を行うチャネルの他に、クライアントはコマンドを送信するときに、サーバーがレスポンスを返すための別チャネルを作成して、その送信側をコマンドに含めてサーバーに送信する。
> サーバーは、コマンドを処理後、コマンドに含まれた送信チャネルを使用してクライアントにレスポンスを返す。
> クライアントは、作成した別チャネルの受信チャネルでサーバーから送信されるレスポンスを受信する。
