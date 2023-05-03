# rust-spotify-search-api

for the spotify access token u need to make an app with their dev tools, then use postman to send a post request with their commands, they got dif comands for dif keys that do dif things (brilliant) 

Heres cli log to build and run if u wanna make it


(base) aleksandersienkiewicz@Aleksanders-MacBook-Air ~ % which rust 

rust not found

(base) aleksandersienkiewicz@Aleksanders-MacBook-Air ~ % rustc 

Usage: rustc [OPTIONS] INPUT

(base) aleksandersienkiewicz@Aleksanders-MacBook-Air ~ % which rustc

/Users/aleksandersienkiewicz/.cargo/bin/rustc

(base) aleksandersienkiewicz@Aleksanders-MacBook-Air ~ % which cargo

/Users/aleksandersienkiewicz/.cargo/bin/cargo

(base) aleksandersienkiewicz@Aleksanders-MacBook-Air ~ % cd documents

(base) aleksandersienkiewicz@Aleksanders-MacBook-Air documents % cd projectdev

(base) aleksandersienkiewicz@Aleksanders-MacBook-Air projectdev % cargo new rust-spotify-search-api

Created binary (application) `rust-spotify-search-api` package

(base) aleksandersienkiewicz@Aleksanders-MacBook-Air projectdev % cd rust-spotify-search-api

(base) aleksandersienkiewicz@Aleksanders-MacBook-Air rust-spotify-search-api % ls

Cargo.toml	src

(base) aleksandersienkiewicz@Aleksanders-MacBook-Air rust-spotify-search-api % cargo run "Hot Rod" "7b460f74ea1c415081f56c1ba43d74df"

Compiling cfg-if v1.0.0

Compiling pin-project-lite v0.2.9

Compiling libc v0.2.142

Compiling futures-core v0.3.28

Compiling bytes v1.4.0

Compiling futures-sink v0.3.28

Compiling smallvec v1.10.0

Compiling scopeguard v1.1.0

Compiling log v0.4.17

Compiling lock_api v0.4.9

Compiling itoa v1.0.6

Compiling slab v0.4.8

Compiling futures-channel v0.3.28

Compiling futures-task v0.3.28

Compiling memchr v2.5.0

Compiling core-foundation-sys v0.8.4

Compiling bitflags v1.3.2

Compiling futures-io v0.3.28

Compiling pin-utils v0.1.0

Compiling once_cell v1.17.1

Compiling fnv v1.0.7

Compiling futures-util v0.3.28

Compiling fastrand v1.9.0

Compiling tinyvec_macros v0.1.1

Compiling http v0.2.9

Compiling hashbrown v0.12.3

Compiling tracing-core v0.1.30

Compiling parking_lot_core v0.9.7

Compiling signal-hook-registry v1.4.1

Compiling mio v0.8.6

Compiling parking_lot v0.12.1

Compiling socket2 v0.4.9

Compiling num_cpus v1.15.0

Compiling errno v0.3.1

Compiling io-lifetimes v1.0.10

Compiling tracing v0.1.37

Compiling rustix v0.37.18

Compiling core-foundation v0.9.3

Compiling security-framework-sys v2.8.0

Compiling tokio v1.28.0

Compiling indexmap v1.9.3

Compiling tinyvec v1.6.0

Compiling lazy_static v1.4.0

Compiling security-framework v2.8.2

Compiling percent-encoding v2.2.0

Compiling try-lock v0.2.4

Compiling want v0.3.0

Compiling form_urlencoded v1.1.0

Compiling serde v1.0.160

Compiling httparse v1.8.0

Compiling http-body v0.4.5

Compiling tempfile v3.5.0

Compiling ryu v1.0.13

Compiling unicode-normalization v0.1.22

Compiling native-tls v0.2.11

Compiling tower-service v0.3.2

Compiling unicode-bidi v0.3.13

Compiling httpdate v1.0.2

Compiling idna v0.3.0

Compiling encoding_rs v0.8.32

Compiling base64 v0.21.0

Compiling futures-executor v0.3.28

Compiling url v2.3.1

Compiling ipnet v2.7.2

Compiling mime v0.3.17

Compiling futures v0.3.28

Compiling serde_urlencoded v0.7.1

Compiling serde_json v1.0.96

Compiling tokio-util v0.7.8

Compiling tokio-native-tls v0.3.1

Compiling h2 v0.3.18

Compiling hyper v0.14.26

Compiling hyper-tls v0.5.0

Compiling reqwest v0.11.17

Compiling rust-spotify-search-api v0.1.0 (/Users/aleksandersienkiewicz/Documents/ProjectDev/rust-spotify-search-api)

Finished dev [unoptimized + debuginfo] target(s) in 15.70s

Running `target/debug/rust-spotify-search-api 'Hot Rod' 7b460f74ea1c415081f56c1ba43d74df`

Need to grab a new token            //LOL this was my bad... 



(base) aleksandersienkiewicz@Aleksanders-MacBook-Air rust-spotify-search-api % cargo run "Hot Rod" "BQCrXt5BOdEoHIfeswrNKk1K1AGY9pujrqlLfha3bEkl0DMmJknfwDceZW6-QQASiGsIpN0D54pPzduTa16PVbt9OGj4VHTdLj3mEQGvSBOid2-
u9zTN"

Finished dev [unoptimized + debuginfo] target(s) in 0.22s

Running `target/debug/rust-spotify-search-api 'Hot Rod' BQCrXt5BOdEoHIfeswrNKk1K1AGY9pujrqlLfha3bEkl0DMmJknfwDceZW6-QQASiGsIpN0D54pPzduTa16PVbt9OGj4VHTdLj3mEQGvSBOid2-u9zTN`

Hot Rod

Fuzzybrain

Dayglow

https://open.spotify.com/track/5euumi7eqEgmxvCIJw2pSp

---------

Hot Rod Lincoln

Lost In The Ozone

Commander Cody and His Lost Planet Airmen

https://open.spotify.com/track/2sZjmnS1BSoc1svOu5P8LM

---------

Hot Rod Hearts - Remastered

Robbie Dupree

Robbie Dupree

https://open.spotify.com/track/1QEZIhNmz28cJ2Lr45MKJ4

---------

Hot Rod Lincoln

Western Standard Time

Asleep At The Wheel

https://open.spotify.com/track/0Hy0uZOxSbOD5gJZRtbLT3

---------

Hot Rod Heart

Blue Moon Swamp

John Fogerty

https://open.spotify.com/track/59LL82V5s2Y86PX3Fvg9Fq

---------

Hot rod

Riff man

Delta Delight

https://open.spotify.com/track/0oYHvGKvrR0pn94pUTApu0

---------

Hot Rod Lincoln

Rockabilly Gold Masters

Various Artists

https://open.spotify.com/track/2FvpvjHX32OLvendFImN9N

---------

Hot Rod Lincoln

Hot to Trot

Commander Cody and His Lost Planet Airmen

https://open.spotify.com/track/68xdfhcAzKgPnU0kv1WY6s

---------

Hot Rod Lincoln

Early Recordings 1957-1962

Roger Miller

https://open.spotify.com/track/48s1GKR5HaOeD3lIiL3Rln

---------

Hot Legs

Seventies Rock

Various Artists

https://open.spotify.com/track/6t4qHyJN6Bt3LJIWTUROhp

---------

The Pharmacist

Sorry About Tomorrow

Hot Rod Circuit

https://open.spotify.com/track/0KclZLYgFPkj4K2bTlPCHF

---------

Mercury Blues

16 Biggest Hits


Alan Jackson

https://open.spotify.com/track/7JiCaZ93B0hdj3XwFqwn4W

---------

455 Rocket

Love Travels

Kathy Mattea

https://open.spotify.com/track/7FtuKCaePFfKqGetBYHxS1

---------

Hot Rod Racer

Greatest Hits 1961 - 1976

Dick Dale & His Del-Tones

https://open.spotify.com/track/0JvJXnvKAdvtZQyzSXGIFP

---------

Hot Legs

Foot Loose & Fancy Free

Rod Stewart

https://open.spotify.com/track/0NIRG3SEn2Zf88o3qHJil5

---------

Hot Rod

Filthydelphia

Ralphy Red

https://open.spotify.com/track/1cFmX9qSbMHbzutGBbRifV

---------

Hot Rod Lincoln

Western Standard Time

Asleep At The Wheel

https://open.spotify.com/track/3w03Q2mmFl1xQ44xHQ7qGU

---------

Hot Rod

The Teaches of Peaches

Peaches

https://open.spotify.com/track/63yGvZ0PoLKTkqCV1EGm0W

---------

Hot Rod Juice

Gee Tee

Gee Tee

https://open.spotify.com/track/4kjyTwhAlELJXVE80fgBrM

---------

Hot Rod

Junkyard

Junkyard

https://open.spotify.com/track/5DR7kwICJYOrEbGlxB0LX4

---------

