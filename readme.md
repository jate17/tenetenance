# Tenetenance


## Abstract

Software modulare per la manutezione di macchine. Velocizzare i processi aziendali di manutezione delle macchine seguendo check-list.

## Obiettivo Attuale

Sviluppo delle seguenti funzioni di base al momento supporto solo su sistemi di base unix.

- Checklist esempio

    - Controllo spazio disco
    - Controllo utilizzo CPU e RAM 
    - Aggiornamento pacchetti di sistema
    - Verifica servizi (es. web server, database)
    - Pulizia file temporanei o log vecchi
    - Controllo backup eseguiti
    - Riavvio VPS (manuale o programmato)


## Funzioni

- Ottenimento info sullo storage ✅
- Ottenimento info sulla ram  ✅
- Ottenimento info sulla cpu  ✅
- Ottenimento info sugli users  ✅
- Ottenimento info sul network  ✅
- Ottenimento info sui i processi ✅
- Verifica di una versione di un software ✅
- Puliza file temporanei ✅
- Puliza log ✅
- Verifica delle porte aperte ✅
- File config YAML 
- Funzioni di Backup 
- Funzionalità simili a winmerge

backup_file(source_path: &str, destination_path: &str) -> Result<()>
Copia un singolo file dalla sorgente alla destinazione, con gestione degli errori.
backup_directory(source_dir: &str, destination_dir: &str) -> Result<()>
Copia ricorsivamente tutti i file e sottodirectory da una cartella sorgente a quella di destinazione.
compress_backup(backup_path: &str, compressed_path: &str) -> Result<()>
Comprimi il contenuto di un backup vecchio (file o directory) in un archivio compresso (ad es. gzip o xz).
verify_backup(file_path: &str, expected_hash: &str) -> Result<bool>
Calcola l’hash di un file di backup e lo confronta con un valore atteso per assicurare integrità.
list_backups(backup_root: &str) -> Result<Vec<String>>
Elenca tutti i backup esistenti in una directory per gestione e selezione.
delete_backup(backup_path: &str) -> Result<()>
Elimina backup vecchi o non necessari per liberare spazio.

## Next 

- Gestione VM
- Windowzz
- Docker 
- API 
- Controllo dei backup 
- Start script di backup custom 
- Verifica firewall rule -> Da fare su macchina con Linux o Ubuntu
