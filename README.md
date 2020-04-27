# A web demo with rust language

> The purpose is to be familiar with the use of rust. For web developers, you can learn from it.

> Welcome to submit an issue in the issue.

> Welcome to submit your excellent code on the branch.

+ The basic web framework is actix-web.
+ Use sqlx as mysql's third party library.
+ Use mobc_redis as the three-party library of redis, built-in connection pool.
+ Use log4rs as the log library.

### Problems
+ log4rs log, currently does not support asynchronous, and does not support kv structure storage, unsightly after json.
+ sqlx does not support orm for now, you need to spell sql manually.
+ Redis does not currently support clusters.
