Some Incus configurations have potentially serious security implications, so I
decided to have `dbd_configure` install incus without initializing it or
otherwise configuring the system to use it. Therefore, it is the end user's
responsibility to initialize/configure incus as appropriate. I am simply
including this file here as a reminder of steps I usually take after installing
incus.

References:
    https://linuxcontainers.org/incus/docs/main/installing/#installing
    https://github.com/ganto/copr-lxc4/wiki/Getting-Started-with-Incus-on-Fedora
    https://linuxcontainers.org/incus/docs/main/howto/network_bridge_firewalld/#network-bridge-firewall

Add a non-root user to the `incus-admin` group:
    usermod -a -G incus-admin <user>

Set ID mapping ranges:
    echo "root:1000000:1000000000" >> /etc/subuid
    echo "root:1000000:1000000000" >> /etc/subgid

Start incus:
    systemctl enable --now incus

Initialize incus:
    incus admin init

Add bridge to trusted zone in firewalld:
    firewall-cmd --zone=trusted --change-interface=incusbr0 --permanent
    firewall-cmd --reload

List images:
    incus image list images:

Start a Fedora 39 container:
    incus launch images:fedora/39
