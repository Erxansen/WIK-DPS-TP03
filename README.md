# WIK-DPS-TP03
TP3 DevOps

Voici la liste des commandes à faire :

1/ "git clone https://github.com/Erxansen/WIK-DPS-TP03"

2/ "docker-compose up"
             
3/ Une fois le docker compose lancé, il reste plus qu'a vérifier si l'API est bien appelée avec la commande "curl localhost:7878/ping -v" qui retourne "HTTP/1.1 200 OK" avec les headers de la requête.

4/ La commande "curl localhost:7878 -v" retourne "HTTP/1.1 404 Not Found" avec "Content-lenght=0" qui annonce une page vide.

5/ Pour vérifier que les réplicas sont bien tous utilisés, il suffit de refresh la page avec "localhost:8080", "localhost:8080/ping" ou rappeler plusieurs fois la commande "curl localhost:7878/ping -v" pour voir dans le terminal où le docker-compose est lancé.