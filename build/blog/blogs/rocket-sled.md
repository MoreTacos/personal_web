# Guide to Using Rocket With Sled in Production
*May 14, 2021*

Alright let's cut right to it. If you're using a different version of rocket (0.4) or sled (0.34) then me then some of this stuff won't be valid. Consider this article a checklist of things you want to consider when building a rocket+sled app.

### Advice 1
Use rocket::ignite().manage(Database{ ... }) when initializing rocket to have access to your database on all routes.

You can add db: State\<Database\> as an argument to whatever route you define to have access to your DB.

### Advice 2
Sled's IVec's are kind of difficult to deal with at first.

The way I like to think of it is that IVec is similar to binary. Basically you gotta explain to sled how to store your structs in the DB.

Some types already have a as_bytes method on them (String for example). But for more complicated structs you have to explain yourself to sled. Thankfully it's quite easy with the bincode library, which you can slap on a struct as an impl method named as_bytes or similar.

Then for deserializing, you need to explain to sled how to go from bytes to Rust types.

Some types already have this implemented (String from_utf8 for example). But for more complicated structs you gotta (again) do it yourself. You can again use bincode for easy deserializing.

Tip: consider implementing Rust's From traits for both ways

### Advice 3
Standardize your database calls by implementing a DatabaseExt trait for State<'_, Database>.

You're constantly going to be needing the same DB information across different routes, so why not standardize it?

Specifically, you need to create a DatabaseExt trait with methods you use constantly like get_user, update_user etc.

Then you implement that trait on the external type (in this case impl DatabaseExt for State<'_, Database>).

And from now on you can call db.get_user() or any other method you've implemented.

## Final words
The truth is that there are very few articles out there explaining the issues that you'll be dealing with on your journey. But that's OK. Figure things out.

If you are really trying to build a production app in Rust+Rocket+Sled, my best advice is go look at code that's already out there and analyze the hell out of it. Take a look at [my repo](https://github.com/MoreTacos/skilltree) or [Dev and dev's repo](https://github.com/alepez/devand) (which I used heavily myself). Go back to previous commits and understand what's going on.

And if you have any questions, feel free to ask the community! (or me)
