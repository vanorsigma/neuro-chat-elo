# TODO
This project is currently in a sorta-kinda-ready-to-use state.
Admittedly, it is still very limited - however, most of the changes required aren't particularly difficult (on their own).

## Tasks
The following tasks are loosely arranged by importance, with the most important tasks being first.

- [ ] Read through the `// TODO: ` and `// FIXME: ` statements in the repo, there's a few of them and they might be important to look at.
- [ ] Leaderboard saving / loading
    - Leaderboards are currently intialized as empty, which means that
        1. Elos are in a bit of a weird state, as everyone has the same elo so winning matches is entirely based on chance (no way to predict who is going to be against who).
        2. When the program exits, all state is thrown out.
    - On the current version of the website, a bunch of user metadata is attached with performances. In my opinion, this is irrelevant to the data being stored - it would probably be a better idea to create a program that can export website-ready leaderboards (or leave this up to the `User state` point if the list, which could also handle filling this user state)
    - Implementation suggestions:
        - In `live-elo`, leaderboards need to be loaded before `UnstartedWebsocketServer` is initalized - ideally they are completely separated.
        - To do file saving, I'd suggest creating a new exporter / replacing `DummyExporter` with something like `FinalFileExporter`, which would collect performances during the run of the program, then save them when cancelled.
            - This would probably involve adding a cleanup function ([`// TODO: There should probably be some kind of way to clean this up in the pipeline...`](https://github.com/owobred/leaderboard-generics/blob/eda40a1b311ba1807bb2c7fb7519db64b93a84e5/live-elo/src/sources.rs#L33-L35)). This would also mean adding a `cleanup` method to `StandardLeaderboard`, `MultiExporter` abd all the other exporters. I'd probably suggest creating a `Cleanupable` trait (probably with a different name) and adding it to the generic requirements of `Pipeline` (or maybe make a newtype over pipeline which has those requirements).
            - Find some kind of way to hoop up `tokio::singnal::ctrl_c` with stopping, maybe by passing each source a [`tokio_util::sync::CancellationToken`](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html) and having it select on that when getting messages.
 - [ ] Check elo calculator implementation
    - Currently this is using [my elo matching algorithm](https://github.com/vanorsigma/neuro-chat-elo/pull/27#issuecomment-2322841347), which I haven't with a properly persisting leaderboard. This means I have no idea if its actually stable if there are many users.
 - [ ] Improve wire protocol
    - Messages are sent over the websocket in JSON currently. This would almost certainly be better if it was sent as something else - like msgpack or protobuf (or something more custom - probably implemented using something like [`bincode`](https://docs.rs/bincode/latest/bincode/)  on the rust side and then hand-writing a parser on the javascript side).
    - Clients currently connect directly to `live-elo`, which is fine but could create a risk of the websockets getting attacked (e.g. some form of DOS) and fully taking down `live-elo` - which would suck. A proposed idea to reduce this risk was to create some kind of middleman between `live-elo` and the clients.
        - To implement this, you basically just need to copy over the logic from `handle_websocket` in `live-elo` into its own seperate program.
        - All the middleman would have to do is:
            1. Connect to `live-elo`
            2. Read the current leaderboards
            3. Apply changes send from `live-elo` and update its copy of leaderboard states
            4. Whenever a client connects, send it the current copy of leaderboard state and then feed it updates (from `live-elo`)
    - Implementation notes:
        - Currently, `live-elo` attempts to avoid serializing messages multiple time (via `Arc<SerializedOutgoingMessage>` which is essentially an `Arc<Vec<u8>>`). This is probably fine as `live-elo` -> middleman communications *should* hold basically no per-connection state. However, if there is any filtering implemented per-connection (`Per-connection filtering` in list) in the middleman the same technique probably won't be usable in the middleman.
 - [ ] User state
    - The websocket only sends user ids and elo information currently[^1]. This is not enough to replicate the current state of the leaderboards which includes more profile information.
    - Potential solutions:
        - Create a REST api which is called when profile information needs to be fetched.
            - Issues:
                - This could potentially be abused if there is no checking done on the user ids sent. A potential solution would be attaching some kind of signature to every user id sent, but this would introduce some bloat/complexity into the websocket protocol (though is a nice solution as it would allow the api to run entirely on cloudflare workers whilst also allowing the results to be cached).
                - If a new user appears in the leaderboard, every client would have to fetch the user from the api. This could cause the backend the exceed ratelimits for fetching user information which would suck.
            - Notes:
                - If combined with `Per-connection filtering` (or even just lazy-loading of profile information depending on what's visible on the client), the risk of every client having to load a new user would be much lower (as the top of the leaderboard *should* stay relatively stable).
                - A decent amount of state is collected by `twitch_irc` in `live-elo`. I'm not confident there's a nice way to provide this to a REST api (nor that it would be more efficient than just fetching it when needed) - but worth pointing out that its available.
        - Send profile data over the websocket:
            - Issues:
                - This would make the websocket have to send massively more data, which is not something we want to do.
                - As this data is sent over the websocket, no caching is provided.
 - [ ] More leaderboards
    - The only existing leaderboard at the moment is the `message_count` leaderboard which just awards a user a fixed number of points for every message they send.
    - The current website has more leaderboards, which could probably be ported over relatively easily.
    - Notes for adding existing leaderboards:
        - You'll probably need to make a few new forms of `AuthorId` for every platform.
        - For implementing the emotes leaderboard, you'll probably have to make another `Source` that takes in a stream of emotes from other sources. You'd probably have to either:
            - Add a bunch of mpscs in the constructors of the other sources, and have them send over any emotes that they see.
            - Create some kind of wrapper over the other exporters, and then inspect their messages (i.e. create a `emote_list(&self) -> Vec<...>` method on `Message`) for emotes. Then, pass through the original message and add the observed emotes to a queue of messages that get sent on the next call to `next_message`.
                - This would probably look something like the following (not an example I've tested, just an idea of how to implement it).
                    ```rust
                    struct TwitchId(String);
                    struct EmoteName(String);

                    enum AuthorId {
                        Twitch(TwitchId),
                        Emote(EmoteName),
                    }

                    struct Emote {
                        name: EmoteName,
                    }

                    enum MyMessageType {
                        Emote(Emote),
                        ...,
                    }

                    impl MyMessageType {
                        fn get_emotes(&self) -> Vec<Emote> { ... }
                    }

                    impl AuthoredMessage for MyMessageType {
                        type AuthorId = AuthorId;

                        fn author_id(&self) -> Self::AuthorId {
                            match self {
                                Emote(emote) => AuthorId::Emote(emote.name.clone()),
                            }
                        }
                    }

                    struct EmoteExtracingSource<S, M> where S: Source<Message = M> {
                        inner_source: S,
                        emote_queue_send: mpsc::Sender<Emote>,
                        emote_queue_recv: mpsc::Receiver<Emote>,
                    }

                    impl<S> Source for EmoteExtracingSource<S, MyMessageType> where S: Source<Message = MyMessageType> {
                        type Message = MyMessageType;

                        async fn next_message(&mut self) -> Option<Self::Message> {
                            tokio::select! {
                                message = inner_source.next_message() => {
                                    if let Some(message) = message {
                                        let emotes = message.get_emotes();
                                        for emote in emotes {
                                            self.emote_queue_send.send(emote).await;
                                        }
                                    }

                                    message
                                },
                                emote = emote_queue_recv.recv() => emote.map(|emote| MyMessageType::Emote(emote)),
                            }
                        }
                    }
                    ```
            - There might be some more obvious solution to this, but I think the main limiting factor is that there is an inherent assumption that one message will have one author and produce one performance (per leaderboard).
 - [ ] Per-connection filtering
    - When updates are sent over the websocket, they include changes for every part of the leaderboard - as well as every leaderboard. This results in a lot of bandwidth being used for changes that will (most likely) never be seen.
    - Ideally, clients would be able send the server some kind of mask which details where it wants to get updates for, allowing then server to only send updates for the region the user is looking at. When they scroll they could then modify this mask to change where they get sent updates (as well as having the server backfill them any changes that they missed).
    - It would be cool to get live updates on your ranking on the leaderboard in real time. This could probably be done by just always sending user updates for a given user id.
        - This *could* involve some kind of authentication system, though it might work if placed into the websocket query params (i.e. `/websocket?track_user=twitch:...`)
    - Implementation suggestions:
        - This will probably involve tracking some state per connection - so I'd suggest doing this in a middleman process (as suggested above). This would mean that the middleman gets full, unfiltered leaderboard changes from `live-elo` and then transforms them to be more relevant to each client.
    - Other notes:
        - I'm not really a web developer, but there must be some way to create an empty table that is sized as if it was full of users, whilst having no actual data in it. If possible it would be really useful for lazy loading.

### Stretch Tasks
These are some tasks that aren't required to get this running, but would be really nice.
 - [ ] Metrics & trace collection
    - Currently, the only way to observe `live-elo` is through the stderr printed logs. Ideally there'd be a way to hook this up to a dashboard or something.
    - Grafana offers a free hosted solution with 14 days of data retention, which would likely fit our needs well.
    - Given the need to collect both traces and metrics, opentelemetry seems like the best option for instrumenting our applications (probably using [node_exporter](https://github.com/prometheus/node_exporter) for host statistics).
    - List of metrics & traces that would be useful:
        - Host statistics:
            - CPU usage
            - Networked I/O usage
            - Disk utilization
            - Memory usage
        - `live-elo` statistics:
            - Messages ingested
            - Elo distributions?
                - This might not be possible to do on the free grafana teir - this would probably require some funky kind of data source which I do not want to have to deal with.
        - Middleman statistics
            - Clients connected
            - Number of messages from `live-elo`
            - Number of messages sent to clients
            - Number of messages read from clients
            - Traces showing which users connected/disconnected from the websocket, and when. (iirc these can be overlayed onto time-series graphs like messages/second which would probably be the best usecase)
    - Considerations:
        - Should metrics be visible / openly scrapable? (i.e. putting up a public `/metrics` endpoint)

[^1]: Technically, its currently sending the username. This is to work around the issue with user data and really shouldn't stay as a solution.