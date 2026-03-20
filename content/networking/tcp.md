---
title: TCP
---

## TCP (Transmission Control Protocol)

A network protocol ensuring reliable, ordered data delivery between any two
machines across a network.

## TCP vs TCP/IP

TCP (Transmission Control Protocol) is a single protocol that operates at the
transport layer — it's responsible for breaking data into packets, ensuring they
arrive in order, and retransmitting lost ones. ​

TCP/IP is not a single protocol but a whole suite (stack) of protocols that work
together to enable internet communication. It's named after its two most
important protocols — TCP and IP — but it also includes UDP, HTTP, DNS, FTP, and
many others.

### The Role of Each Layer

TCP/IP is organized into four layers, each with a distinct job: ​

- Application layer — user-facing protocols like HTTP, FTP, SMTP, DNS

- Transport layer — TCP or UDP (handles delivery, ordering, error checking)

- Internet layer — IP (handles addressing and routing packets to the right
  machine)

- Network Access layer — physical transmission over Ethernet, Wi-Fi, etc.

### How TCP and IP Work Together

IP and TCP are complementary: IP figures out where to send data (routing between
machines using IP addresses), while TCP figures out how to send it reliably
(ordering, retransmission, flow control). TCP doesn't send data directly to the
recipient — it hands packets down to IP, which routes them across the network.

A simple analogy: IP is like the postal system that routes packages to the right
address, and TCP is the careful courier who numbers each box, confirms delivery,
and resends anything that got lost.

So in short: TCP is one piece inside TCP/IP. When people say "TCP/IP," they mean
the entire networking framework that powers the internet.
