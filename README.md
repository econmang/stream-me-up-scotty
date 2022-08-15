# Stream Me Up, Scotty

## Table of Contents
* [Introduction](#Introduction)
* [Technologies](#Technologies)

## Introduction
There are many streaming services available today. Some are really neat and have a lot to offer... others have issues of only having a limited set of options of interest.
But, to get those, you have to add their subscription to your monthly bills. 

This application is meant to help avoid those additional costs. The goal is to have this be a streaming application to be used against a home media server.
Users could pay a one-time cost of the digital media and then avoid a subscription cost beyond that point.

They could store their library on a PC and then use this interface to create profiles, watchlists, etc. against that personal library.

## Technologies
This is just a rough outline, so I'm still trying to determine the stack. I would like to use Rust. If the interface becomes web-oriented or is deployed as a service
as opposed to a local application, I'm hoping to incorporate WASM.

Current thoughts on stack:

Desktop Application:

MySQL: Contain movie and resource location data.
MongoDB: Storage of user information, including watch lists and rating history.
Tauri: Utilizing Vue3 to create the front-end and Rust for the back-end.
Note: The host system would need network access to whatever home device contained the films / the local media server

I may end up creating a web service iteration of this; may consider web hosting and creating a purchasing layer to host people's movie library.
In that case, I'd be interested in attempting to incorporate WASM as part of this project.
