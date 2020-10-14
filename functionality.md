# Discord Counting Bot Functionality Breakdown

1. Start a "session" in a channel (probably at 1/0)
2. One user at a time may increment the value in a message (any text after the number is valid)
3. If a user gets the number wrong, they are "eliminated" from the session
4. If a user does not send a message for some period of time (either specified or calculated from heuristics), then are "eliminated" also
5. The last user with access to the channel who is not "eliminated" is the winner

## Ideas

- DM people to say they're eliminated if they try to participate after losing?
- Multiple channels?
- Leaderboard/hall of fame? Perhaps guild-wide?
- 1 match: 1 channel

## Implementation

- Use Redis backend with persistence enabled
- Store current number for each channel as something like `{guild_id}/{channel_id}/count`
- Store participating players as something like `{guild_id}/{channel_id}/players`
- Store per-guild settings as something like `{guild_id}/settings` with serde json
