using System;
using System.Collections.Generic;
using System.Speech.Synthesis;

namespace rockzombot2
{
    class rockzombot2
    {
        // Bot settings
        private static string _botName = "rockzombot2";
        private static string _broadcasterName = "rockzombie2";
        private static string _twitchOAuth = "oauth:ncmdjdvlgiif9ywhnlbbop6wr8udjh"; // get chat bot's oauth from www.twitchapps.com/tmi/

        static void Main(string[] args)
        {
            // Initialize variables
            List<Zombie> zombies = new List<Zombie>();
            SpeechSynthesizer reader = new SpeechSynthesizer();

            // Initialize and connect to Twitch chat
            IrcClient irc = new IrcClient("irc.twitch.tv", 6667,
                _botName, _twitchOAuth, _broadcasterName);

            // Ping to the server to make sure this bot stays connected to the chat
            // Server will respond back to this bot with a PONG (without quotes):
            // Example: ":tmi.twitch.tv PONG tmi.twitch.tv :irc.twitch.tv"
            PingSender ping = new PingSender(irc);
            ping.Start();
            
            // Listen to the chat until program exits
            while (true)
            {
                // Read any message from the chat room
                string message = irc.ReadMessage();
                Console.WriteLine(message); // Print raw irc messages

                if (message.Contains("PRIVMSG"))
                {
                    // Messages from the users will look something like this (without quotes):
                    // Format: ":[user]![user]@[user].tmi.twitch.tv PRIVMSG #[channel] :[message]"

                    // Modify message to only retrieve user and message
                    int intIndexParseSign = message.IndexOf('!');
                    string userName = message.Substring(1, intIndexParseSign - 1); // parse username from specific section (without quotes)
                                                                                   // Format: ":[user]!"
                    // Get user's message
                    intIndexParseSign = message.IndexOf(" :");
                    message = message.Substring(intIndexParseSign + 2);

                    //Console.WriteLine(message); // Print parsed irc message (debugging only)

                    // Broadcaster commands
                    if (userName.Equals(_broadcasterName))
                    {
                        if (message.Equals("!exitbot"))
                        {
                            irc.SendPublicChatMessage("Bye! Have a beautiful time!");
                            Environment.Exit(0); // Stop the program
                        }

                        if (message.StartsWith("!feed"))
                        {
                            String[] msg = message.Split(' ');
                            if (msg.Length > 1)
                            {
                                String username = msg[1];
                                Zombie zombie = zombies.Find(x => x.getUsername() == username);

                                if (zombie != null)
                                {
                                    int brainz = 0;
                                    if (msg.Length > 2)
                                    {
                                        brainz = Int32.Parse(msg[2]);
                                        zombie.addBrainz(brainz);
                                    } else
                                    {
                                        brainz = 1;
                                        zombie.addBrainz(brainz);
                                    }

                                    irc.SendPublicChatMessage(zombie.getUsername() + " was fed " + brainz +
                                        " brain" + (brainz > 1 ? "z" : "")+ "!\r" +
                                        "["+zombie.getUsername()+" has "+zombie.getBrainz()+
                                        " brain" + (zombie.getBrainz() > 1 ? "z" : "")+ "!]");
                                } else
                                {

                                }
                            }
                        }

                        if (message.StartsWith("!add"))
                        {
                            String[] msg = message.Split(' ');
                            if (msg.Length > 1)
                            {
                                String username = msg[1];
                                Zombie zombie = new Zombie(username);

                                zombies.Add(zombie);
                                irc.SendPublicChatMessage(zombie.getUsername() + " was added to the zombie horde!");
                            }
                        }


                    }

                    // General commands anyone can use
                    if (message.Equals("!hello"))
                    {
                        irc.SendPublicChatMessage("Hello World!");
                    }

                    if (message.Equals("!brainz") || message.Equals("!brains") || message.Equals("!brain"))
                    {
                        Zombie zombie = zombies.Find(x => x.getUsername() == userName);
                        if (zombie != null)
                        {
                            irc.SendPublicChatMessage("["+zombie.getUsername()+" has "+zombie.getBrainz()+
                                " brain" + (zombie.getBrainz() > 1 ? "z" : "")+ "!]");
                        }
                    }

                    if (message.Equals("!elo"))
                    {
                        irc.SendPublicChatMessage("Silver V");
                    }

                    if (message.StartsWith("!say"))
                    {
                        Zombie zombie = zombies.Find(x => x.getUsername() == userName);
                        if (zombie != null)
                        {
                            if (zombie.getBrainz() >= 3)
                            {
                                zombie.eatBrainz(3);

                                irc.SendPublicChatMessage("[" + zombie.getUsername() + " ate 3 brainz!]");

                                String say = message.Substring(4);
                                reader.Volume = 100;
                                reader.SpeakAsync(say);
                            }
                        }
                    }

                } else if (message.Contains("JOIN"))
                {
                    Console.WriteLine("JOINED: " + message);
                    // Messages from the users will look something like this (without quotes):
                    // Format: ":[user]![user]@[user].tmi.twitch.tv PRIVMSG #[channel] :[message]"

                    // Modify message to only retrieve user and message
                    int intIndexParseSign = message.IndexOf('!');
                    string userName = message.Substring(1, intIndexParseSign - 1); // parse username from specific section (without quotes)
                                                                                   // Format: ":[user]!"
                    // Get user's message
                    intIndexParseSign = message.IndexOf(" :");
                    message = message.Substring(intIndexParseSign + 2);

                    Zombie newZombie = new Zombie(userName);
                    newZombie.addBrainz(1);
                    zombies.Add(newZombie);

                    irc.SendPublicChatMessage("Welcome, " + newZombie.getUsername() + "!");
                    //irc.SendPublicChatMessage("Welcome to the channel, "+newZombie.getUsername()+
                    //    "! You have been granted 1 free brain!\r["+newZombie.getUsername()+
                    //    " has "+newZombie.getBrainz()+" brain"+
                    //    (newZombie.getBrainz() > 1 ? "z" : "")+"!]");
                }
            }
        }
    }
}

