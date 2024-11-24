import os
import pty
import sys
import fcntl
import termios
import struct
import tty
import select

import time

cat = [[
    "⠀⠀⡄⠀⣀⣰⣳⡃⠀",
    "⠰⠦⠈⠭⠛⠛⠛⠇⠀"
],
[
    "⠀⠀⠀⢀⠀⣤⣴⣳⡃",
    "⠸⠥⡄⠀⠩⠛⠛⠛⠅"
],
[
    "⠀⠀⠀⢀⠀⣠⣤⣞⣞",
    "⠀⠀⠀⠌⠉⠙⠟⠻⠋"
],
[
    "⢀⠀⢀⣀⣞⣞⠀⠀⠀",
    "⠀⠉⠹⠛⠛⠟⠀⠀⠀"
]]
cnt = 0
def catBar(rows, cols):
    global cnt
    cnt = (cnt + 1) % 4
    text = ""
    text += "\033[s"
    text += "\033[0m\033[30m\033[47m"
    text += f"\033[r\033[{rows-1};{1}H"
    text += f"{cat[cnt][0]}" + (" " * (cols-9))
    text += f"\n\r{cat[cnt][1]}" + (" " * (cols-9))
    text += f"\033[1;{rows-2}r"
    text += f"\033[0m"
    text += f"\033[u"
    return text


cols, rows = os.get_terminal_size()

pid = os.fork()
if pid != 0:    #親　猫が走る
    while 1:
        time.sleep(0.1)
        text = catBar(rows, cols)
        os.write(sys.stdout.fileno(), text.encode())
        pid, _ = os.waitpid(pid, os.WNOHANG)
        if pid != 0:
            break

else:
    master_fd, slave_fd = pty.openpty()

    size = struct.pack("HHHH", rows-2, cols, 0, 0)
    fcntl.ioctl(slave_fd, termios.TIOCSWINSZ, size)

    pid = os.fork()
    if pid == 0:
        os.close(master_fd)
        os.setsid()
        os.dup2(slave_fd, sys.stdin.fileno())
        os.dup2(slave_fd, sys.stdout.fileno())
        os.dup2(slave_fd, sys.stderr.fileno())
        if slave_fd > sys.stderr.fileno():
            os.close(slave_fd)
        os.execlp("bash", "bash")
    else:  # 親プロセス
        os.close(slave_fd)
        old_tty = termios.tcgetattr(sys.stdin)
        tty.setraw(sys.stdin.fileno())
        tty.setcbreak(sys.stdin.fileno())
        try:
            while True:
                r, w, e = select.select([sys.stdin, master_fd], [], [])
                if master_fd in r:
                    data = os.read(master_fd, 10240)
                    # os.write(sys.stdout.fileno(), f"\033[1;{rows-2}r".encode())
                    os.write(sys.stdout.fileno(), data)
                if sys.stdin in r:
                    data = os.read(sys.stdin.fileno(), 10240)
                    os.write(master_fd, data)


        except KeyboardInterrupt:
            pass
        finally:
            termios.tcsetattr(sys.stdin, termios.TCSADRAIN, old_tty)
            os.close(master_fd)
            print(f"\033[r")
