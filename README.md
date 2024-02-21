# TER_Rust_implementation
Une implémentation en Rust de GROUP_BY et MERGE_SORT
## Installer Rust sur un système Unix
Exécuter en ligne de commande


```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```

Tester l'installation avec


```rustup --version```

En cas de ```command not found``` vérifier que ~/.cargo/bin est bien sur le PATH
## Utilisation de cargo
Se placer dans le directory TER_Rust_implementation puis :
* Pour lancer les tests :  ```cargo test```
* Pour exécuter la fonction main de src/main : ```cargo run```
* Pour lancer la compilation sans exécuter le main : ```cargo build```
* Pour vérifier que le code compile sans déclencher la compilation : ```cargo check```
