# 🔧 Tenetenance

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Status](https://img.shields.io/badge/status-in%20development-yellow?style=for-the-badge)

> Software modulare per la manutenzione automatizzata di macchine e server

## 📋 Panoramica

Tenetenance è un tool modulare progettato per velocizzare i processi aziendali di manutenzione delle macchine attraverso checklist automatizzate e un sistema di gestione centralizzato.

**Attualmente supportato:** Sistemi Unix/Linux

## 🎯 Obiettivi del Progetto

Il software permette di eseguire operazioni di manutenzione ordinaria e straordinaria tramite checklist predefinite:

- ✅ Controllo spazio disco
- ✅ Monitoraggio utilizzo CPU e RAM
- ✅ Aggiornamento pacchetti di sistema
- ✅ Verifica servizi attivi (web server, database)
- ✅ Pulizia file temporanei e log obsoleti
- ✅ Controllo stato backup
- ✅ Gestione riavvio VPS (manuale o programmato)

## ⚡ Funzionalità Implementate

### Monitoraggio Sistema
- ✅ Informazioni storage (spazio disco, partizioni)
- ✅ Informazioni RAM (utilizzo, disponibilità)
- ✅ Informazioni CPU (carico, core)
- ✅ Informazioni utenti attivi
- ✅ Informazioni rete (interfacce, connessioni)
- ✅ Elenco processi in esecuzione

### Diagnostica
- ✅ Verifica versione software installato
- ✅ Controllo connessioni
- ✅ Analisi stato servizi

### Manutenzione
- ✅ Pulizia file temporanei
- ✅ Pulizia log obsoleti
- ✅ Sistema di logging integrato

### Backup
- ✅ Creazione backup
- ✅ Checksum e verifica integrità file
- ✅ Sincronizzazione backup

## 🐛 Bug Conosciuti



## 🚀 Roadmap

### In Sviluppo
- [ ] API REST per controllo remoto
- [ ] Dashboard web per gestione centralizzata

### Pianificato
- [ ] Gestione macchine virtuali (VM)
- [ ] Supporto Windows
- [ ] Integrazione Docker
- [ ] Controllo avanzato backup con notifiche
- [ ] Esecuzione script di backup personalizzati
- [ ] Verifica regole firewall

## 🛠️ Stack Tecnologico

- **Linguaggio:** Rust
- **Backend API:** Actix-web
- **Serializzazione:** Serde


## 🏗️ Architettura

Il progetto è strutturato come Cargo workspace per modularità e riutilizzo del codice:

```

tenetenance/
├── Cargo.toml # Workspace root
├── Cargo.lock
│
├── cli/ # Command-line interface
│ ├── Cargo.toml
│ └── src/
│ ├── main.rs
│
├── server/ # API server (C2)
│ ├── Cargo.toml
│ └── src/
│ ├── main.rs
│

```


## 🔐 Sicurezza

Se trovi vulnerabilità di sicurezza, per favore **NON** aprire issue pubbliche. Contatta direttamente il maintainer.


## 👤 Autore

**[Il tuo nome]**

- GitHub: [@jate17](https://github.com/jate17)

---

<div align="center">

**⚠️ Nota:** Il progetto è in fase di sviluppo attivo. Alcune funzionalità potrebbero non essere complete o stabili.


[Report Bug](https://github.com/jate17/tenetenance/issues) · [Request Feature](https://github.com/jate17/tenetenance/issues) · [Documentation](https://github.com/jate17/tenetenance/wiki) ASAP Disponibile  

</div>