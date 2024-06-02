# Shiny

It's a Discord bot written in rust lang using [poise framework](https://docs.rs/poise/latest/poise/index.html). It can create channels, delete channels. You can test this bot by using it in your server [bot link](https://discord.com/oauth2/authorize?client_id=1243906255425110098&permissions=8&scope=bot)

#### Functions 

* After adding first time in the server it will make important roles and channels to function (don't forget to move the bot in the top in roels).
    * _It will make following categories_
    * **can_create_channel** : To check if user is allow to create channels (give this role to users that you want to be able to create channel).
    * **has_active_channel** : To check if any user has active channels.
    * _It will creates following channels_
    * **Invitiations** : It will be ivitations category under which there will be channel where user can post their channel invitation so intrested memebers can join there channels.
* It creates Private channel for users (user must have can create_channel_role to make channels) using `/create_channel` command and it take channel type as an optional arguement, by defualt it will make text channel. And the bot will dm the user their invite token of channel and they can invite people using their token to their room.
* Users can use `/post <title> <message>` command to post their message in `invites` channel and add their title and message and optionally then can add the roles they want to ping.
* Other users of the server can use `/add <token>` command to add to the channel.
* And user can delete their channel using `/delete` command.


#### TODO
* Delete invite after some time.
* Delete channels after some time of inactivity.
* Ability to add other users using `/add_user <users>` command.
* Implement better token system for channels.
* Give user the ability to delete their posts or edit their posts using `/delete_post` `/edit_post` commands.
