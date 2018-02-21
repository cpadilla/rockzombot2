using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace rockzombot2
{
    class Zombie
    {
        int brainz = 0;
        String username;

        public Zombie(String username)
        {
            this.username = username;
        }

        public void addBrainz(int amount)
        {
            brainz += amount;
        }

        public void eatBrainz(int brainz)
        {
            this.brainz -= brainz;
        }

        public int getBrainz()
        {
            return brainz;
        }
        
        public String getUsername()
        {
            return username;
        }
    }
}
