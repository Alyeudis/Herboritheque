use serde::Deserialize;
use std::fs;

// Modèle de données (moule) représentant une plante sur la base de la fiche technique 
#[derive(Debug, Deserialize)]
struct Plante {
    id: String,
    nom_commun: String,
    nom_latin: String,
    famille: String,
    zone_cueillette: String,
    periode_cueillette: String,
}

fn main() {
    // Charger le contenu du template dans une variable
    let chemin = "./examples/template.md"; // Chemin d'accès du template. 
    let contenu = fs::read_to_string(chemin)
        .expect("Erreur : Impossible de lire le fichier !");

    // Extraire le bloc YAML (entre les deux premiers ---) des fiches techniques
    // Découpage de la fiche technique pour isoler le YAML à prendre en compte par rust
    let parties: Vec<&str> = contenu.split("---").collect();
    
    // Vérifier que le fichier contient bien les séparateurs --- du bloc YAML
    if parties.len() >= 3 {
        let yaml_brut = parties[1];
        
        // Transformer le YAML en structure Rust exploitable
        let plante: Plante = yaml_serde::from_str(yaml_brut)
            .expect("Erreur : Le format YAML ne correspond pas à la structure !");

        // Afficher les informations de la fiche technique
        println!(" Clé d'identification : {}", plante.id);
        println!(" Nom de la plante : {} ({})", plante.nom_commun, plante.nom_latin);
        println!(" Famille : {}", plante.famille);
        println!(" Zone de cueillette : {}",plante.zone_cueillette);
        println!(" Période de cueillette : {}", plante.periode_cueillette);
    } else {
        println!(" Erreur : Le fichier ne semble pas avoir de bloc YAML (délimité par ---)");
    }
}

