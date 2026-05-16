use lopdf::Dictionary;
use std::collections::BTreeMap;
use lopdf::content::{Content, Operation};
use lopdf::{Document, Object, ObjectId, Stream, Bookmark};
use std::path::Path;
use colored::*;
use std::process::{Command, Output};
use std::path::PathBuf;
use dirs;


pub fn merge_pdf(path1: String, path2: String) -> Result<(), Box<dyn std::error::Error>>{
    let p1 = Path::new(&path1);
    let p2 = Path::new(&path2);

    if !p1.is_file(){
        return Err(format!("{} is not found/ a file", path1).red().into());
    }

    if !p2.is_file(){
        return Err(format!("{} is not found / a file", path2).red().into())
    }

    let doc1 = Document::load(p1)?;
    let doc2 = Document::load(p2)?;

    let documents = vec![
        doc1,
        doc2
    ];

    let mut max_id = 1;
    let mut pagenum = 1;
    // Collect all Documents Objects grouped by a map
    let mut documents_pages = BTreeMap::new();
    let mut documents_objects = BTreeMap::new();
    let mut document = Document::with_version("1.5");

    for mut doc in documents {
        let mut first = false;
        doc.renumber_objects_with(max_id);

        max_id = doc.max_id + 1;

        documents_pages.extend(
            doc
                    .get_pages()
                    .into_iter()
                    .map(|(_, object_id)| {
                        if !first {
                            let bookmark = Bookmark::new(String::from(format!("Page_{}", pagenum)), [0.0, 0.0, 1.0], 0, object_id);
                            document.add_bookmark(bookmark, None);
                            first = true;
                            pagenum += 1;
                        }

                        (
                            object_id,
                            doc.get_object(object_id).unwrap().to_owned(),
                        )
                    })
                    .collect::<BTreeMap<ObjectId, Object>>(),
        );
        documents_objects.extend(doc.objects);
    }

    // "Catalog" and "Pages" are mandatory.
    let mut catalog_object: Option<(ObjectId, Object)> = None;
    let mut pages_object: Option<(ObjectId, Object)> = None;

    // Process all objects except "Page" type
    for (object_id, object) in documents_objects.iter() {
        // We have to ignore "Page" (as are processed later), "Outlines" and "Outline" objects.
        // All other objects should be collected and inserted into the main Document.
        match object.type_name().unwrap_or(b"") {
            b"Catalog" => {
                // Collect a first "Catalog" object and use it for the future "Pages".
                catalog_object = Some((
                    if let Some((id, _)) = catalog_object {
                        id
                    } else {
                        *object_id
                    },
                    object.clone(),
                ));
            }
            b"Pages" => {
                // Collect and update a first "Pages" object and use it for the future "Catalog"
                // We have also to merge all dictionaries of the old and the new "Pages" object
                if let Ok(dictionary) = object.as_dict() {
                    let mut dictionary = dictionary.clone();
                    if let Some((_, ref object)) = pages_object {
                        if let Ok(old_dictionary) = object.as_dict() {
                            dictionary.extend(old_dictionary);
                        }
                    }

                    pages_object = Some((
                        if let Some((id, _)) = pages_object {
                            id
                        } else {
                            *object_id
                        },
                        Object::Dictionary(dictionary),
                    ));
                }
            }
            b"Page" => {}     // Ignored, processed later and separately
            b"Outlines" => {} // Ignored, not supported yet
            b"Outline" => {}  // Ignored, not supported yet
            _ => {
                document.objects.insert(*object_id, object.clone());
            }
        }
    }

    // If no "Pages" object found, abort.
    if pages_object.is_none() {
        println!("Pages root not found.");

        return Ok(());
    }

    // Iterate over all "Page" objects and collect into the parent "Pages" created before
    for (object_id, object) in documents_pages.iter() {
        if let Ok(dictionary) = object.as_dict() {
            let mut dictionary = dictionary.clone();
            dictionary.set("Parent", pages_object.as_ref().unwrap().0);

            document
                    .objects
                    .insert(*object_id, Object::Dictionary(dictionary));
        }
    }

    // If no "Catalog" found, abort.
    if catalog_object.is_none() {
        println!("Catalog root not found.");

        return Ok(());
    }

    let catalog_object = catalog_object.unwrap();
    let pages_object = pages_object.unwrap();

    // Build a new "Pages" with updated fields
    if let Ok(dictionary) = pages_object.1.as_dict() {
        let mut dictionary = dictionary.clone();

        // Set new pages count
        dictionary.set("Count", documents_pages.len() as u32);

        // Set new "Kids" list (collected from documents pages) for "Pages"
        dictionary.set(
            "Kids",
            documents_pages
                    .into_iter()
                    .map(|(object_id, _)| Object::Reference(object_id))
                    .collect::<Vec<_>>(),
        );

        document
                .objects
                .insert(pages_object.0, Object::Dictionary(dictionary));
    }

    // Build a new "Catalog" with updated fields
    if let Ok(dictionary) = catalog_object.1.as_dict() {
        let mut dictionary = dictionary.clone();
        dictionary.set("Pages", pages_object.0);
        dictionary.remove(b"Outlines"); // Outlines not supported in merged PDFs

        document
                .objects
                .insert(catalog_object.0, Object::Dictionary(dictionary));
    }

    document.trailer.set("Root", catalog_object.0);

    // Update the max internal ID as wasn't updated before due to direct objects insertion
    document.max_id = document.objects.len() as u32;

    // Reorder all new Document objects
    document.renumber_objects();

    // Set any Bookmarks to the First child if they are not set to a page
    document.adjust_zero_pages();

    // Set all bookmarks to the PDF Object tree then set the Outlines to the Bookmark content map.
    if let Some(n) = document.build_outline() {
        if let Ok(Object::Dictionary(dict)) = document.get_object_mut(catalog_object.0) {
            dict.set("Outlines", Object::Reference(n));
        }
    }

    document.compress();

    // Save the merged PDF.
    // Store file in current working directory.
    // Note: Line is excluded when running doc tests
    if true {
        document.save("merged.pdf").unwrap();
        println!("Done")
    }

    Ok(())
}

fn downloads_dir() -> anyhow::Result<PathBuf> {
    let path = dirs::download_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find Downloads folder"))?;
    Ok(path)
}

pub fn convert_pdf (path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let outdir = downloads_dir()?;
    let args = [
        "--headless",
        "--infilter=writer_pdf_import",  // writer not impress
        "--convert-to", "odt",
        "--outdir", outdir.to_str().unwrap(),
        path.to_str().unwrap()
    ];

    let out = Command::new("libreoffice").args(args).output()?;


    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(format!("LibreOffice failed: {}", stderr).into());
    }

    let odt_path = outdir
        .join(path.file_stem().unwrap())
        .with_extension("odt");

    let out = Command::new("libreoffice")
        .args([
            "--headless",
            "--convert-to", "docx",
            "--outdir", outdir.to_str().unwrap(),
            odt_path.to_str().unwrap(),
        ])
        .output()?;

    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(format!("LibreOffice step 2 failed: {}", stderr).into());
    }

    // Clean up the intermediate ODT file
    std::fs::remove_file(&odt_path)?;

    let output = outdir
        .join(path.file_stem().unwrap())
        .with_extension("docx");

    println!("Done! The file is saved to {}", output.display().to_string().green());
    Ok(())

}