use colored::*;
use rand::Rng;
use rusqlite::{params, Connection, Result};
use std::{io, str::FromStr};

#[derive(Debug)]
struct StructTable {
    id: u8,
    name: String,
    points: u8,
    date: String,
    time: String,
    dateline: String,
}

fn main() {
    let mut nb_essaie = 5;
    let random = rand::thread_rng().gen_range(1..100);
    println!("{}", "\n###############################\n# WELCOME - game more or less #\n###############################\n".bold().blue());

    println!("Régles du jeu : vous devez saisir un nombre compris entre 1 et 100, si vous trouvez le bon nombre vous gagnez, si au bout de cinq tentatives vous n'avez pas trouvé le bon nombre, vous perdez !\n\n");

    let username = request_username();
    while nb_essaie != 0 {
        match request_number() {
            Some(nb) => {
                if nb > random {
                    println!("{}", "=> C'est plus petit !\n".red());
                } else if nb < random {
                    println!("{}", "=> C'est plus grand !\n".red());
                } else {
                    println!("{}", "=> Félicitation, vous avez GAGNE ;)".blue());
                    break;
                }
            }
            None => {}
        }
        nb_essaie -= 1;
        if nb_essaie == 0 {
            return println!(
                "{} {}",
                "\n\nVous avez perdu...\nLe nombre à trouver était le numéro".red(),
                random
            );
        }
    }
    create(username, nb_essaie);
    println!("\n\nSouhaitez-vous voir le tableau des résultats ? (Y/N)");
    let mut touche = String::new();
    io::stdin()
        .read_line(&mut touche)
        .expect("Erreur lors de la lecture de votre saisie.");
    match String::from_str(&touche.trim()) {
        Ok(t) => {
            if t == "y" || t == "Y" {
                println!("\n");
                read();
            }
        }
        Err(_) => println!("Ne peut vérifier votre saisie."),
    }
    println!("\nA bientôt ;)");
}

fn request_username() -> String {
    println!("\nSaisissez votre speudo :");
    loop {
        let mut username = String::new();
        io::stdin()
            .read_line(&mut username)
            .expect("Erreur lors de la lecture de votre saisie.");
        let username = match username.trim().parse() {
            Ok(user) => user,
            Err(_) => "Impossible de supprimé les espaces autour de votre saisie !".to_owned(),
        };

        if bdd_user().unwrap().iter().any(|x| x == &username) == true {
            println!("Ce speudo existe déja !\nSaisissez un autre speudo : ");
        } else {
            break username;
        }
    }
}

fn request_number() -> Option<usize> {
    loop {
        println!("Saisissez un nombre : ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Impossible de récupérer votre saisie.");
        match usize::from_str(&input.trim()) {
            Ok(nb) => {
                if nb < 1 || nb > 100 {
                    println!("Veuillez saisir un nombre valide compris entre 1 et 100 : ")
                } else {
                    break Some(nb);
                }
            }
            Err(_) => {
                println!("Veuillez entrer un nombre valide.")
            }
        };
    }
}

fn bdd_user() -> Result<Vec<String>> {
    let conn = Connection::open("bdd.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS table_ranking (
            ID integer PRIMARY KEY,
            name text NOT NULL UNIQUE,
            points integer NOT NULL,
            date DATE NOT NULL DEFAULT (strftime('%d/%m/%Y', 'now', 'localtime')),
            time TIME NOT NULL DEFAULT (strftime('%H:%M', 'now', 'localtime')),
            dateline DATE NOT NULL DEFAULT (strftime('%d/%m/%Y', 'now', 'localtime', '+2 MONTH')) 
        );",
        [],
    )?;

    let mut stmt = conn.prepare("SELECT name FROM table_ranking")?;
    let rows = stmt.query_map([], |row| row.get(0))?;

    let mut names_bdd = Vec::<String>::new();
    for i in rows {
        names_bdd.push(i?);
    }

    Ok(names_bdd)
}

fn create(txt: String, points: u8) -> Result<()> {
    let conn = Connection::open("bdd.db")?;

    conn.execute("DELETE FROM table_ranking WHERE dateline >= date;", [])?;
    conn.execute(
        "INSERT INTO table_ranking (name, points) VALUES (?1, ?2)",
        params![txt, points],
    )?;

    Ok(())
}

fn read() -> Result<()> {
    let conn = Connection::open("bdd.db")?;

    let mut prepare_table =
        conn.prepare("SELECT * FROM table_ranking ORDER BY points DESC LIMIT 10")?;
    let iter_table = prepare_table.query_map([], |row| {
        Ok(StructTable {
            id: row.get(0)?,
            name: row.get(1)?,
            points: row.get(2)?,
            date: row.get(3)?,
            time: row.get(4)?,
            dateline: row.get(5)?,
        })
    })?;

    println!("La table 'StructTable' contient les éléments suivant : \n");
    for index in iter_table {
        println!(
            "{:#?}\n",
            index.expect("Erreur : impossible de lire la table !")
        );
    }

    Ok(())
}
