# Практика Docker
## 1. Установить docker на виртуалку из первого задания (для проверки работоспособности можете запустить контейнер hello-world: https://hub.docker.com/_/hello-world)
[Install using the apt repository](https://docs.docker.com/engine/install/ubuntu/)
## 2. Далее, возьмите любое приложение с просторов интернета (подойдет любой рабочий сервис с вашего личного гитхаба; если сервиса у вас нет, то возьмите один из репозитория https://github.com/somnoynadno/hits-docker-practice) и разместите в любой директории. Важным критерием для проекта должна быть зависимость от внешней базы данных (не sqlite), потому что во второй части лабы часть баллов будет засчитана за правильную настройку контейнера с БД.
```sh
git clone https://github.com/somnoynadno/hits-docker-practice
```
## 3. После этого вам потребуется cоздать `Dockerfile` для сборки своего приложения (то есть пошаговую "инструкцию" для создания контейнера); файл необходимо разместить в корне проекта, чтобы сборка инициировалась командой $ docker build .

### Для дополнительных баллов:
В качестве базового сборочного образа использовать любую alpine-сборку, найденную на `https://hub.docker.com/`
Через директиву LABEL указать maintainer'а (свои контакты) (достаточно фамилию + имя)
Перебить дефолтного пользователя контейнера с использованием директивы USER
Определить в процессе сборки переменную окружения “ENVIRONMENT” со значением “stage”
Использовать в процессе сборки `multi-stage build`, состоящий из двух шагов
Разместить в корневой директории `.dockerignore`, чтобы запретить попадание в контейнер файла docker-compose.yml, содержимого директории .git, а также всех файлов с расширением .env и .md
Установить и запустить hadolint для проверки наличия грубых ошибок в вашем Dockerfile; если таковые имеются, то их требуется исправить (важно: hadolint должен находиться в PATH и быть исполняемым для всех)
4. Создать `docker-compose.yml` в корне проекта, в котором:
Указать версию спецификации не ниже третьей
Настроить контейнер с БД:
[DB] Дать сервису назваие db (иначе дальше проверка не пойдет)
[DB] Найти на `https://hub.docker.com/` образ БД с тегом latest и включить его в docker-compose.yml файл
[DB] Настроить название хоста равное 'database_host'
[DB] Передавать все креды для запуска через директиву environment
[DB] Через директиву expose указать порт БД (при этом сама БД наружу торчать не должна, то есть директиву ports использовать не нужно)
[DB] Настроить volumes для персистентного хранения данных на диске
Настроить сборку своего приложения:
[APP] Дать ему название app (иначе дальше проверка не пойдет)
[APP] Использовать в сборке приложения `Dockerfile` из первой части лабы
[APP] Прокинуть наружу порт `80`
[APP] Cделать порт `80` доступным `только` для хостовой машины
Настроить виртуальную сеть между контейнером приложения и БД, чтобы они могли друг с другом общаться
Явно установить подходящую политику для рестарта контейнеров
5. После того как всё готово, скачайте и запустите чекер для своей платформы с параметрами `$ docker-checker -name 'Ваше Имя' -project /path/to/project`, указав вторым параметром путь к проверяемому проекту




Nim Legacy
```
brew install libpq
echo 'export PATH="/opt/homebrew/opt/libpq/bin:$PATH"' >> ~/.zshrc

ln -s /opt/homebrew/Cellar/libpq/17.2/bin/psql /usr/local/bin/psql


```
# For compilers to find libpq you may need to set:
export LDFLAGS="-L/opt/homebrew/opt/libpq/lib"
export CPPFLAGS="-I/opt/homebrew/opt/libpq/include"
# For pkg-config to find libpq you may need to set:
export PKG_CONFIG_PATH="/opt/homebrew/opt/libpq/lib/pkgconfig"



export LIBRARY_PATH=/opt/homebrew/lib
PATH=/opt/homebrew/bin:$PATH
export LD_LIBRARY_PATH=/opt/homebrew/lib:$LD_LIBRARY_PATH
```

nim c --passL:"-Wl,-no_warn_duplicate_libraries" -d:nimDebugDlOpen dbtest.nim



--dynlibOverrid /opt/homebrew/opt/

nim --app:lib --dynlibOverride:libsqlite3 --passC:'-I$(HEADERS)' --passL:'-L$(LIB) -lsqlite3' c blah.nim
nimble run -v --dynlibOverride:libpq --passL:'-L$(LIB) -lsqlite3'
```
