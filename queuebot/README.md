# QueueBot

QueueBot is a Discord bot used for CS 120's office hours. It allows students to join a which TAs can then pull students off of when they are available.
Old CS 120 QueueBot code: <https://github.com/benperumala/cs120-queuebot/>

This repo has a mockup bot which does the same thing but without connecting to Discord's API (so it's easier to run and see how it works).
There are Rust crates that [allow for communication with Discord's API](https://github.com/SpaceManiac/discord-rs) so it is possible to extend this to an actual product

This bot takes an IRC-like approach to commands where a user can type a message and then the bot responds.

## Student Commands

Anyone can run these commands

`!q ping` - Bot responds with "Pong!" (Used to check to see if the bot is working)  
`!q join` - Add the current user to the queue  
`!q leave` - Remove the current user from the queue  
`!q position` - Get the position of the current user within the queue  
`!q list` - List all students within the queue  

## TA Commands

Only TAs are allowed to run these commands

`!q next` - Pop the next student from the queue  
`!q clear` - Clear/empty the queue  
`!q add @user` - Add the specified `@user` to the end of the queue  
`!q remove @user` - Remove the specified `@user` from the queue  
