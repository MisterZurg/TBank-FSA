# Практика Linux
> [!note]
> ```bash
> monsterzerg@denis:~/Downloads$ sudo chmod 777 sysadmin-lab-linux-checker-linux-arm64-v0.5.4
> monsterzerg@denis:~/Downloads$ ./sysadmin-lab-linux-checker-linux-arm64-v0.5.4 --help
> Usage of ./sysadmin-lab-linux-checker-linux-arm64-v0.5.4:
>   -name string
>     	your surname and name (e.g. 'Зоркин Александр')
> ```

В рамках практики по виртуализации мы установили виртуальную машину с образом Ubuntu под VirtualBox.

Теперь на данной виртуальной машине вам необходимо выполнить следующие задания,
залогинившись под пользователем `root`:
```sh
sudo su -
whoami
```

```sh
pwd
# /root
```

Обновить установленные пакеты через пакетный менеджер `apt`
```sh
sudo apt-get update && sudo apt-get install
```

Дополнительно установить пакеты `vim`, `nano`, `tree` и `traceroute` через пакетный менеджер `apt` (они вам дальше помогут)
```sh
sudo apt install vim -y nano tree traceroute
```

Создать пользователя с именем `user` и паролем `strongpassword321`
```sh
# cоздание нового пользователя в системе
sudo adduser user
# смена пароля для пользователя
sudo passwd user strongpassword321
```

Создать группу с названием `admin` и добавить туда пользователя `user`
```sh
# создание новой группы
sudo addgroup admin
# The "-a" (append) switch is essential. Otherwise, the user will be removed from any groups, not in the list.
# The "-G" or "--groups" switch takes a (comma-separated) list of additional groups to assign the user to.
sudo usermod -a --groups admin user
```

Разрешить пользователю `user` выполнять команды от лица суперпользователя, добавив его в группу sudo через утилиту usermod
```sh
sudo usermod -aG sudo user
```

Создать и открыть на редактирование файл /root/nirvana-song.txt, записать в него текст песни “Smells Like Teen Spirit” группы Nirvana
```sh
vim nirvana-song.txt
```
<details>
<summary>🤘🗿🎸</summary>

```toml
[Verse 1]
Load up on guns, bring your friends
It's fun to lose and to pretend
She's over-bored and self-assured
Oh no, I know a dirty word

[Pre-Chorus]
Hello, hello, hello, how low
Hello, hello, hello, how low
Hello, hello, hello, how low
Hello, hello, hello

[Chorus]
With the lights out, it's less dangerous
Here we are now, entertain us
I feel stupid and contagious
Here we are now, entertain us
A mulatto, an albino
A mosquito, my libido, yeah

[Post-Chorus]
Hey, yay

[Verse 2]
I'm worse at what I do best
And for this gift, I feel blessed
Our little group has always been
And always will until the end
See upcoming rock shows
Get tickets for your favorite artists
You might also like
Red Button
Drake
The Shoe Fits
Drake
You Broke My Heart
Drake
[Pre-Chorus]
Hello, hello, hello, how low
Hello, hello, hello, how low
Hello, hello, hello, how low
Hello, hello, hello

[Chorus]
With the lights out, it's less dangerous
Here we are now, entertain us
I feel stupid and contagious
Here we are now, entertain us
A mulatto, an albino
A mosquito, my libido, yeah

[Post-Chorus]
Hey, yay

[Guitar Solo]

[Verse 3]
And I forget just why I taste
Oh yeah, I guess it makes me smile
I found it hard, it's hard to find
Oh well, whatever, never mind

[Pre-Chorus]
Hello, hello, hello, how low
Hello, hello, hello, how low
Hello, hello, hello, how low
Hello, hello, hello
[Chorus]
With the lights out, it's less dangerous
Here we are now, entertain us
I feel stupid and contagious
Here we are now, entertain us
A mulatto, an albino
A mosquito, my libido

[Outro]
A denial, a denial
A denial, a denial
A denial, a denial
A denial, a denial
A denial
```

</details>


```sh
# vim nirvana-song.txt
:wq
```


Создать директорию /root/new-dir с правами 740, установить владельцем созданной директории пользователя user и группу admin
```sh
mkdir -p new-dir
# chmod 740 new-dir
chmod -R u+rwx,g+r--,o+--- ./new-dir

chown -R user:admin ./new-dir
```

Создать симлинк (symbolic link) /usr/local/bin/ls-link на файл /bin/ls, проверить работоспособность
```sh
ln -s /bin/ls /usr/local/bin/ls-link
```

Создать переменную окружения с названием MY_ENV и значением BEST_LINUX_LAB_EVER
```sh
export MY_ENV=BEST_LINUX_LAB_EVER
```

Установить веб-сервер nginx через пакетный менеджер apt и запустить его (убедиться, что он слушает стандартный порт 80)
```sh
sudo apt install nginx
systemctl status nginx
# ctrl + c
```

Вывести список файлов и каталогов в директории /etc, перенаправить (записать) его в файл /root/etc-directories-list.txt
> [!note]
> `>` перенаправляет stdout в файл


```sh
ls -al /etc > ./etc-directories-list.txt
```

Посчитать количество файлов (НЕ каталогов!) в директории /bin, перенаправить это число в файл /root/bin-files-count.txt
```sh
# ls -l ../bin | grep -v ^l | wc -l > /bin-files-count.txt
find /bin -type f | wc -l > ./bin-files-count.txt
```

Найти самый большой файл в директории /etc, записать абсолютный путь к нему в /root/biggest-file.txt (подсказка: поиск нужно выполнить по всем поддиректориям)
```sh
find /etc -type f -exec du -h {} + | sort -rh | head -n 1 | awk '{print $2}' > ./biggest-file.txt
```

Создать cronjob, которая каждый день один раз в 5:45 утра будет перезагружать (restart) веб-сервер nginx (кек)
```sh
crontab -e

45 5 * * *  restart nginx

crontab -l
```

Посчитать кол-во доступных сетевых интерфейсов, записать их количество (цифру) в файл /root/interfaces-count.txt
```sh
ip link show | grep -c '^[0-9]' > ./interfaces-count.txt
```

Написать небольшой shell-скрипт /root/script.sh, который создаст ровно 404 пустых файла с разными названиями в директории /root/404/, сделать его исполняемым из любой директории (добавить шебанг) и запустить его ровно один раз
```sh
mkdir -p ./404

vim ./script.sh

```

```bash
#!/bin/bash
for i in $(seq 1 404); do
    touch /root/404/file_$i.txt
done
```
```sh
chmod +x ./script.sh

./script.sh
```

> Посчитать sha256-хешсумму от файла /etc/hosts и записать её в файл /root/sha256-hosts.txt
> ```sh
> sha256sum /etc/hosts > ./sha256-hosts.txt
> ```

Посчитать md5-хешсумму от файла /etc/hosts и записать её в файл /root/md5-hosts.txt
```sh
md5sum /etc/hosts > ./md5-hosts.txt
```

С помощью утилиты ufw разрешить входящие соединения только на порты 22, 80 и 443 и запретить на все остальные, а также явно заблокировать весь входящий трафик с хоста 192.168.101.5
```sh
sudo apt-get install ufw

sudo ufw reset


sudo ufw allow 22/tcp
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw allow 8080/tcp

sudo ufw default deny incoming

sudo ufw deny from 192.168.88.5

sudo ufw enable

sudo ufw status verbose
```

Запустить утилиту traceroute до хоста 8.8.4.4, убедиться, что трафик его достигает, и записать результат выполнения в файл /root/traceroute.txt
```sh
# Трассировка маршрута до IP-адреса 8.8.4.4, который собсга DNS Google.
traceroute 8.8.4.4 > ./traceroute.txt

cat ./traceroute.txt
```

Сгенерировать пару ssh-ключей алгоритмом RSA для пользователя root (не используя passphrase), сохранить их в директорию по умолчанию
```sh
ssh-keygen -t rsa -b 4096 -N ""
```

Отправить GET-запрос к сайту https://example.com через curl, записать заголовки ответа сервера в файл /root/example-curl.txt (найдите опцию утилиты, которая отобразит вам эти заголовки)
```sh
curl -D /root/example-curl.txt https://example.com
```

Проверить количество свободного места на диске в корневом разделе,
записать результат в Гб в файл /root/available-space.txt,
округлив до десятых или до целого числа (например, 17G или 9,6G; если русская локаль, то 9.6G); если места на диске меньше одного Гб, то запишите ответ в Мб (по типу 182M)
```sh
df -h / | awk 'NR==2 {if ($4+0 < 1) printf "%.0fM\n", $4*1024; else printf "%.1fG\n", $4}' > /root/available-space.txt
```


Установить timezone равным Asia/Novosibirsk
```sh
sudo timedatectl set-timezone Asia/Novosibirsk
```


> [!WARN}
```
[INFO] Verifying /root/bin-files-count.txt...
[WARN] Check failed: wrong or unexpected result


[INFO] Verifying /root/md5-hosts.txt...
[WARN] Check failed: wrong or unexpected result


Удобно
```
root@denis:cd ../home/monsterzerg/Downloads/
root@denis:/home/monsterzerg/Downloads# ./sysadmin-lab-linux-checker-linux-arm64-v0.5.4 -name 'Zakharov Denis'

root@denis:/home/monsterzerg/Downloads# cd ../../../root
```


По ссылке https://disk.yandex.ru/d/j9zQGKBHPvah1w доступны чекеры для данного задания. Чекер в автоматическом режиме проверит корректность выполнения ваших заданий и озвучит вам оценку.

Чекер необходимо скачать на виртуалку
При скачивании выберите исполняемый файл под свою архитектуру: x86 или arm.
При запуске чекера нужно указать свои имя и фамилию параметрами командой строки (бинарь сам вам подскажет формат, если вы запустите его с опцией --help)
Запускайте чекер под пользователем root, иначе ему может не хватить прав на чтение файлов
Мы не ограничивали кол-во попыток, но большое количество отправок вызовет у нас подозрение и вопросы
Пожалуйста, не пересоздавайте виртуалку при выполнении лабы, иначе чекер может отработать некорректно и занизить вашу оценку
Если у вас упала одна из проверок, но при этом вы уверены, что вы выполнили всё необходимое, то попробуйте выполнить этот шаг ещё раз (возможно, в системе что-то поменялось и вам надо актуализировать информацию для проверки)
Если чекер работает откровенно плохо, то свяжитесь с преподавателем
