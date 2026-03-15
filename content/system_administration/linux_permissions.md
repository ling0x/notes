---
title: Linux Permissions
---

### Question: What happens if I am on a server and then I ssh into this server again?

Nothing special or dangerous happens by itself; you just get another shell on
the same machine.

Nothing special or dangerous happens by itself; you just get another shell on
the same machine

You now have two nested sessions:

- outer: your original SSH (or local) shell

- inner: the SSH session created from the server to itself.

Processes in the inner session are separate from the outer one, with their own
TTY, environment, history, etc.

### Question: how to create a user on an archlinux server without sudo privilege but make it a member of docker group?

#### Create the New User

`useradd -m -G docker -s /bin/bash newusername`

#### Set a Password

`passwd newusername`

#### Verify Group Membership

```bash
id newusername
# Expected: uid=1001(newusername) gid=1001(newusername) groups=1001(newusername),999(docker)
```

#### Generate the Key Pair

```bash
ssh-keygen -t ed25519 -C "newusername@yourserver" -f /home/newusername/.ssh/id_ed25519 -N ""
```

#### Set Up authorized_keys

```bash
cat /home/newusername/.ssh/id_ed25519.pub >> /home/newusername/.ssh/authorized_keys
```

#### Fix Permissions

SSH is strict about permissions — it will silently refuse to use keys with wrong
ownership or modes:

```bash
chown -R newusername:newusername /home/newusername/.ssh
chmod 700 /home/newusername/.ssh
chmod 600 /home/newusername/.ssh/authorized_keys
chmod 600 /home/newusername/.ssh/id_ed25519
```

#### Share the Private Key

```bash
ssh -i ~/.ssh/id_ed25519 newusername@your_server_ip
```

#### Setup Authorized Keys

Confirm authorized_keys matches that key

```bash
sudo -u deploy cat /home/deploy/.ssh/ed25519.pub
sudo -u deploy cat /home/deploy/.ssh/authorized_keys
```

Ensure authorized_keys contains the same line as ed25519.pub. If it doesn’t,
append it:

```bash
sudo -u deploy sh -c 'cat ~/.ssh/ed25519.pub >> ~/.ssh/authorized_keys'
chmod 600 /home/deploy/.ssh/authorized_keys
```

#### Test SSH Access

```bash
ssh -i /path/to/private_key -p SSH_PORT SSH_USER@SSH_HOST
```
