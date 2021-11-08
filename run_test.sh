#!/usr/bin/sh


trap 'echo "Encerrando processos filhos" && (ps -s $$ -o pid=)| xargs -r kill' EXIT

cd server && cargo build --release && cd .. || exit
cd hashmap && cargo build --release && cd .. || exit
cd client && cargo build --release && cd .. || exit

n=4
echo Criando $((1<<n)) servidores...
for ((i=0; i<1<<n;i++))
do
	server/target/release/server $n $i &
done

# testa uma inserção e uma consulta
client/target/release/client 1 3 i melhorUni PUC-Rio &
client/target/release/client 2 7 c melhorUni 0 &
sleep 1

# testa varias consultas em nós diferetes
client/target/release/client 3 0 c melhorUni 0 &
client/target/release/client 4 15 c melhorUni 0 &
client/target/release/client 5 8 c melhorUni 0 &
client/target/release/client 6 13 c melhorUni 0 &
client/target/release/client 7 1 c melhorUni 0 &
client/target/release/client 8 5 c melhorUni 0 &
sleep 1

# testa a inserção após consulta
client/target/release/client 9 3 c melhorArea 0 &
sleep 1
client/target/release/client 10 3 i melhorArea ConcDist &


echo "Encerrando processos para novos testes com 32 servidores"
pkill -P $$


n=5
echo Criando $((1<<n)) servidores...
for ((i=0; i<1<<n;i++))
do
	server/target/release/server $n $i &
done

# testa uma inserção e uma consulta
client/target/release/client 1 3 i melhorUni PUC-Rio &
client/target/release/client 2 20 c melhorUni 0 &
sleep 1

# testa varias consultas em nós diferetes
client/target/release/client 3 0 c melhorUni 0 &
client/target/release/client 4 31 c melhorUni 0 &
client/target/release/client 5 15 c melhorUni 0 &
client/target/release/client 6 28 c melhorUni 0 &
client/target/release/client 7 1 c melhorUni 0 &
client/target/release/client 8 5 c melhorUni 0 &
sleep 1

# testa a inserção após consulta
client/target/release/client 9 3 c melhorArea 0 &
sleep 1
client/target/release/client 10 3 i melhorArea ConcDist &

sleep 2
exit
