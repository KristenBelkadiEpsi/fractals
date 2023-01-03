use image::{Rgb, RgbImage};
use num::Complex;
// programme principale dessinant l'ensemble de Julia pour un certain point: 0.285 + 0.013i
fn main() {
    //bornes de la fractal
    let x0 = -0.2;
    let x1 = 0.2;
    let y0 = -0.2;
    let y1 = 0.2;

    //module max du nombre complexe
    let module_max = 2.0;

    //nombre d'itérations max de la suite complexe
    let it_max = 1000;

    //longueur (axe x) et hauteur (axe y) de l'image
    let longueur = 10000;
    let hauteur = 10000;
    //on créer l'image correspondante
    let mut img = RgbImage::new(longueur, hauteur);

    //on boucle sur les coordonnées y et x de l'image
    for y in 0..hauteur {
        println!("{} / {}", y, hauteur - 1);
        for x in 0..longueur {
            //on convertit les coordonnées y et x de l'image en coordonnées du plan complexe (compris entres les bornes [x0; x1] et [y0; y1] avec l'origine du repère centrée au milieu de l'image

            let re = ((x as f64) - (longueur as f64 / 2.0)) * (x1 - x0) / (longueur as f64);
            let im = ((y as f64) - (hauteur as f64 / 2.0)) * (y1 - y0) / (hauteur as f64);

            //on definit la suite complexe : Z(n) = Z(n-1) ^ 2 + c
            //avec Z(0) = re + im * i et c = 0.285 + 0.013i
            //et on arrête cette suite quand on dépasse le module max definit plus haut ou qu'on excède le nombre d'itérations max
            let mut z = Complex::new(re, im);
            let c = Complex::new(0.285, 0.013);
            let mut n = 0;
            while n < it_max && z.norm() < module_max {
                z = z * z + c;
                n += 1;
            }
            //lorsqu'on sort de cette suite en dépassant le nombre d'itérations max on colore le pixel (x,y) en noir
            if n >= it_max {
                img.put_pixel(x, y, Rgb([0, 0, 0]))
            }
            //lorsqu'on sort de cette suite en dépassant le module max on colore le pixel (x,y) en une teinte de rouge dont l'intensité est proportionnelle au nombre d'itérations
            if z.norm() >= module_max {
                img.put_pixel(
                    x,
                    y,
                    Rgb([(255.0 * (n as f64) / (it_max as f64)) as u8, 0, 0]),
                );
            }
        }
    }
    //on sauvegarde l'image au format bmp
    img.save("image.bmp").unwrap();
}
