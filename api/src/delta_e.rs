pub struct CIEDE2000;
// pub struct Lab {
//     pub l: f32,
//     pub a: f32,
//     pub b: f32,
// }
use lab::Lab;
pub struct Coefficients {
    pub ksub_l: f32,
    pub ksub_c: f32,
    pub ksub_h: f32,
}

impl CIEDE2000 {
    pub fn calc_delta_e(color1: &Lab, color2: &Lab, coefficients: &Coefficients) -> f32 {
        let c_ab_1 = (color1.a.powi(2) + color1.b.powi(2)).sqrt();
        let c_ab_2 = (color2.a.powi(2) + color2.b.powi(2)).sqrt();
        let c_ab_bar = (c_ab_1 + c_ab_2) / 2.0;
        let g = 0.5 * (
            1.0 - (c_ab_bar.powi(7) / (c_ab_bar.powi(7) + (25f32).powi(7))).sqrt()
            );
        let a_prime_1 = (1.0 + g) * color1.a;
        let a_prime_2 = (1.0 + g) * color2.a;
    
        let c_prime_1 = (a_prime_1.powi(2) + color1.b.powi(2)).sqrt();
        let c_prime_2 = (a_prime_2.powi(2) + color2.b.powi(2)).sqrt();
        let h_prime_1 = calc_h_prime(color1.b, a_prime_1);
        let h_prime_2 = calc_h_prime(color2.b, a_prime_2);

        // calculate delta L prime
        let delta_l_difference_prime = color2.l - color1.l;
    
        // calculate delta C prime
        let delta_c_difference_prime = c_prime_2 - c_prime_1;
    
        // calculate delta Hue Angle prime
        let delta_h_prime = calc_delta_h_prime(c_prime_1, c_prime_2, h_prime_1, h_prime_2);
    
        // calculate delta Hue Difference prime
        let delta_h_difference_prime = 2.0 * (c_prime_1 * c_prime_2).sqrt() * (delta_h_prime / 2.0).to_radians().sin();
    
        // calculate L bar prime
        let l_bar_prime = (color1.l + color2.l) / 2.0;
    
        // calculate C bar prime
        let c_bar_prime = (c_prime_1 + c_prime_2) / 2.0;
    
        // calculate h bar prime
        let h_bar_prime = calc_h_bar_prime(h_prime_1, h_prime_2);
    
        // calculate T
        let t =
            1.0 -
            0.17 * (h_bar_prime - 30.0).to_radians().cos() +
            0.24 * (2.0 * h_bar_prime).to_radians().cos() +
            0.32 * (3.0 * h_bar_prime + 6.0).to_radians().cos() - 
            0.20 * (4.0 * h_bar_prime - 63.0).to_radians().cos();
    
    
        // calculate delta theta
        let delta_theta = 30.0 * (-((h_bar_prime - 275.0) / 25.0).powi(2)).exp();
    
        // calculate R sub C
        let r_sub_c = 2.0 * (c_bar_prime.powi(7) / (c_bar_prime.powi(7) + (25.0_f32).powi(7))).sqrt();
    
        // calculate S sub L
        let s_sub_l = 1.0 + (0.015 * (l_bar_prime - 50.0).powi(2)) / (20.0 + (l_bar_prime - 50.0).powi(2)).sqrt();
    
        // calculate S sub C
        let s_sub_c = 1.0 + 0.045 * c_bar_prime;
    
        // calculate S sub H
        let s_sub_h = 1.0 + 0.015 * c_bar_prime * t;
    
        // calculate R sub T
        let r_sub_t = -(2.0 * delta_theta.to_radians()).sin() * r_sub_c;
    
        // calculate delta E
        let delta_e = (
            (delta_l_difference_prime / (coefficients.ksub_l * s_sub_l)).powi(2) + 
            (delta_c_difference_prime / (coefficients.ksub_c * s_sub_c)).powi(2) + 
            (delta_h_difference_prime / (coefficients.ksub_h * s_sub_h)).powi(2) +
            r_sub_t * (delta_c_difference_prime / (coefficients.ksub_c * s_sub_c)) * (delta_h_difference_prime / (coefficients.ksub_h * s_sub_h))
            ).sqrt();

        // stdout all the variables
        // 改行も含めて
        // println!("------------------------------------------------");
        // println!(" c_ab_1: {} \n c_ab_2: {} \n c_ab_bar: {} \n g: {} \n a_prime_1: {} \n a_prime_2: {} \n c_prime_1: {} \n c_prime_2: {} \n h_prime_1: {} \n h_prime_2: {} \n delta_l_difference_prime: {} \n delta_c_difference_prime: {} \n delta_h_prime: {} \n delta_h_difference_prime: {} \n l_bar_prime: {} \n c_bar_prime: {} \n h_bar_prime: {} \n t: {} \n delta_theta: {} \n r_sub_c: {} \n s_sub_l: {} \n s_sub_c: {} \n s_sub_h: {} \n r_sub_t: {} \n delta_e: {}", c_ab_1, c_ab_2, c_ab_bar, g, a_prime_1, a_prime_2, c_prime_1, c_prime_2, h_prime_1, h_prime_2, delta_l_difference_prime, delta_c_difference_prime, delta_h_prime, delta_h_difference_prime, l_bar_prime, c_bar_prime, h_bar_prime, t, delta_theta, r_sub_c, s_sub_l, s_sub_c, s_sub_h, r_sub_t, delta_e);
        // println!("------------------------------------------------");
        

        return delta_e;
    }
    pub fn calc_delta_e_from_rgb(rgb1: &[u8; 3], rgb2: &[u8; 3], coefficients: &Coefficients) -> f32 {
        let color1 = Lab::from_rgb(rgb1);
        let color2 = Lab::from_rgb(rgb2);
        return CIEDE2000::calc_delta_e(&color1, &color2, coefficients);
    }
}

fn calc_h_prime(b_star: f32, a_prime: f32) -> f32 {
    if b_star == 0.0 && a_prime == 0.0 {
        return 0.0;
    }

    let mut h_prime = b_star.atan2(a_prime).to_degrees();
    if h_prime < 0.0 {
        h_prime += 360.0;
    }

    h_prime
}

fn calc_delta_h_prime(c_prime_1: f32, c_prime_2: f32, h_prime_1: f32, h_prime_2: f32) -> f32 {
    if c_prime_1 * c_prime_2 == 0.0 {
        return 0.0;
    }

    let mut delta_h_prime = h_prime_2 - h_prime_1;
    if (h_prime_2 - h_prime_1).abs() <= 180.0 {
        delta_h_prime = delta_h_prime;
    } else if h_prime_2 - h_prime_1 > 180.0 {
        delta_h_prime -= 360.0;
    } else {
        delta_h_prime += 360.0;
    }

    delta_h_prime
}

fn calc_h_bar_prime(h_prime_1: f32, h_prime_2: f32) -> f32 {
    let h_defference_abs = (h_prime_1 - h_prime_2).abs();
    let sum_h_prime = h_prime_1 + h_prime_2;
    let mut h_bar_prime = sum_h_prime / 2.0;

    if h_defference_abs > 180.0 {
        h_bar_prime = (h_prime_1 + h_prime_2 + 360.0) / 2.0;
    }

    h_bar_prime
}

#[cfg(test)]
mod tests {
    use super::{CIEDE2000, Lab, Coefficients};

    fn round(val: f32) -> f32 {
        let rounded = val * 10000 as f32;
        rounded.round() / 10000 as f32
    }

    fn assert_delta_e(expected: f32, lab_1: &[f32; 3], lab_2: &[f32; 3]) {

        let coefficients = Coefficients {
            ksub_l: 1.0,
            ksub_c: 1.0,
            ksub_h: 1.0,
        };

        let color_1 = Lab {
            l: lab_1[0],
            a: lab_1[1],
            b: lab_1[2],
        };

        let color_2 = Lab {
            l: lab_2[0],
            a: lab_2[1],
            b: lab_2[2],
        };

        assert_eq!(round(CIEDE2000::calc_delta_e(&color_1, &color_2, &coefficients)), expected);

    }

    #[test]
    fn tests() {
        assert_delta_e(0.0, &[0.0, 0.0, 0.0], &[0.0, 0.0, 0.0]);
        assert_delta_e(0.0, &[99.5, 0.005, -0.010], &[99.5, 0.005, -0.010]);
        assert_delta_e(100.0, &[100.0, 0.005, -0.010], &[0.0, 0.0, 0.0]);
        assert_delta_e(2.0425, &[50.0000, 2.6772, -79.7751], &[50.0000, 0.0000, -82.7485]);
        assert_delta_e(2.8615, &[50.0000, 3.1571, -77.2803], &[50.0000, 0.0000, -82.7485]);
        assert_delta_e(3.4412, &[50.0000, 2.8361, -74.0200], &[50.0000, 0.0000, -82.7485]);
        assert_delta_e(1.0000, &[50.0000, -1.3802, -84.2814], &[50.0000, 0.0000, -82.7485]);
        assert_delta_e(1.0000, &[50.0000, -1.1848, -84.8006], &[50.0000, 0.0000, -82.7485]);
        assert_delta_e(1.0000, &[50.0000, -0.9009, -85.5211], &[50.0000, 0.0000, -82.7485]);
        assert_delta_e(2.3669, &[50.0000, 0.0000, 0.0000], &[50.0000, -1.0000, 2.0000]);
        assert_delta_e(2.3669, &[50.0000, -1.0000, 2.0000], &[50.0000, 0.0000, 0.0000]);
        assert_delta_e(7.1792, &[50.0000, 2.4900, -0.0010], &[50.0000, -2.4900, 0.0009]);
        assert_delta_e(7.1792, &[50.0000, 2.4900, -0.0010], &[50.0000, -2.4900, 0.0010]);
        assert_delta_e(7.2195, &[50.0000, 2.4900, -0.0010], &[50.0000, -2.4900, 0.0011]);
        assert_delta_e(7.2195, &[50.0000, 2.4900, -0.0010], &[50.0000, -2.4900, 0.0012]);
        assert_delta_e(4.8045, &[50.0000, -0.0010, 2.4900], &[50.0000, 0.0009, -2.4900]);
        assert_delta_e(4.7461, &[50.0000, -0.0010, 2.4900], &[50.0000, 0.0011, -2.4900]);
        assert_delta_e(4.3065, &[50.0000, 2.5000, 0.0000], &[50.0000, 0.0000, -2.5000]);
        assert_delta_e(27.1492, &[50.0000, 2.5000, 0.0000], &[73.0000, 25.0000, -18.0000]);
        assert_delta_e(22.8977, &[50.0000, 2.5000, 0.0000], &[61.0000, -5.0000, 29.0000]);
        assert_delta_e(31.9030, &[50.0000, 2.5000, 0.0000], &[56.0000, -27.0000, -3.0000]);
        assert_delta_e(19.4535, &[50.0000, 2.5000, 0.0000], &[58.0000, 24.0000, 15.0000]);
        assert_delta_e(1.0000, &[50.0000, 2.5000, 0.0000], &[50.0000, 3.1736, 0.5854]);
        assert_delta_e(1.0000, &[50.0000, 2.5000, 0.0000], &[50.0000, 3.2972, 0.0000]);
        assert_delta_e(1.0000, &[50.0000, 2.5000, 0.0000], &[50.0000, 1.8634, 0.5757]);
        assert_delta_e(1.0000, &[50.0000, 2.5000, 0.0000], &[50.0000, 3.2592, 0.3350]);
        assert_delta_e(1.2644, &[60.2574, -34.0099, 36.2677], &[60.4626, -34.1751, 39.4387]);
        assert_delta_e(1.2630, &[63.0109, -31.0961, -5.8663], &[62.8187, -29.7946, -4.0864]);
        assert_delta_e(1.8731, &[61.2901, 3.7196, -5.3901], &[61.4292, 2.2480, -4.9620]);
        assert_delta_e(1.8645, &[35.0831, -44.1164, 3.7933], &[35.0232, -40.0716, 1.5901]);
        assert_delta_e(2.0373, &[22.7233, 20.0904, -46.6940], &[23.0331, 14.9730, -42.5619]);
        assert_delta_e(1.4146, &[36.4612, 47.8580, 18.3852], &[36.2715, 50.5065, 21.2231]);
        assert_delta_e(1.4441, &[90.8027, -2.0831, 1.4410], &[91.1528, -1.6435, 0.0447]);
        assert_delta_e(1.5381, &[90.9257, -0.5406, -0.9208], &[88.6381, -0.8985, -0.7239]);
        assert_delta_e(0.6377, &[6.7747, -0.2908, -2.4247], &[5.8714, -0.0985, -2.2286]);
        assert_delta_e(0.9082, &[2.0776, 0.0795, -1.1350], &[0.9033, -0.0636, -0.5514]);
    }
}