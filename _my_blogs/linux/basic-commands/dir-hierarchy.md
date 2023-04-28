root 下最重要的子目录：

/bin Contains ready-to-run programs (also known as executables), including most of the basic Unix commands such as ls and cp. Most of the programs in /bin are in binary format, having been created by a C compiler, but some are shell scripts in modern systems.

/dev Contains device files. 

/etc This core system configuration directory (pronounced EHT-see) contains the user password, boot, device, networking, and other setup files.

/home Holds home (personal) directories for regular users. Most Unix installations conform to this standard.

/lib An abbreviation for library, this directory holds library files con- taining code that executables can use. There are two types of libraries: static and shared. The /lib directory should contain only shared librar- ies, but other lib directories, such as /usr/lib, contain both varieties
as well as other auxiliary files.

/proc Provides system statistics through a browsable directory-and-file interface. Much of the /proc subdirectory structure on Linux is unique, but many other Unix variants have similar features. The /proc directory contains information about currently running processes as well as some kernel parameters.

/run Contains runtime data specific to the system, including certain process IDs, socket files, status records, and, in many cases, system log- ging. This is a relatively recent addition to the root directory; in older systems, you can find it in /var/run. On newer systems, /var/run is a sym- bolic link to /run.

/sys This directory is similar to /proc in that it provides a device and system interface. You’ll read more about /sys in Chapter 3.

/sbin The place for system executables. Programs in /sbin directories relate to system management, so regular users usually do not have /sbin components in their command paths. Many of the utilities found here don’t work if not run as root.

/tmp A storage area for smaller, temporary files that you don’t care much about. Any user may read to and write from /tmp, but the user may not have permission to access another user’s files there. Many programs use this directory as a workspace. If something is extremely important, don’t put it in /tmp because most distributions clear /tmp when the machine boots and some even remove its old files periodi- cally. Also, don’t let /tmp fill up with garbage because its space is usually shared with something critical (the rest of /, for example).

/usr Although pronounced “user,” this subdirectory has no user files. Instead, it contains a large directory hierarchy, including the bulk of the Linux system. Many of the directory names in /usr are the same as those in the root directory (like /usr/bin and /usr/lib), and they hold the same type of files. (The reason that the root directory does not contain the complete system is primarily historic—in the past, it was to keep space requirements low for the root.)

/var The variable subdirectory, where programs record information that can change over the course of time. System logging, user tracking, caches, and other files that system programs create and manage are here. (You’ll notice a /var/tmp directory here, but the system doesn’t wipe it on boot.)