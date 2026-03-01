
const OFFSET: f32 = 100000.0;

#[derive(Clone)]
pub struct Noise3D
{
        pub scale: f32,
        pub seed: i64,
}

impl Noise3D
{
        fn sample(&self, x: f32, y: f32, z: f32, seed: i64) -> f32
        {
                let mut n = (x as i64).wrapping_mul(73856093)
                        ^ (y as i64).wrapping_mul(19349663)
                        ^ (z as i64).wrapping_mul(83492791)
                        ^ seed;
                n = n.wrapping_mul(n.wrapping_mul(60493).wrapping_add(19990303));
                n = n ^ (n >> 19);
                let val = ((n & 0x7fffffff) as f32) / 1073741824.0;
                val - 1.0
        }

        fn smooth_noise_3d(&self, x: f32, y: f32, z: f32, seed: i64) -> f32
        {
                let int_x = x.floor() as i64;
                let int_y = y.floor() as i64;
                let int_z = z.floor() as i64;
                let frac_x = x - int_x as f32;
                let frac_y = y - int_y as f32;
                let frac_z = z - int_z as f32;
                let u = frac_x * frac_x * (3.0 - 2.0 * frac_x);
                let v = frac_y * frac_y * (3.0 - 2.0 * frac_y);
                let w = frac_z * frac_z * (3.0 - 2.0 * frac_z);
                let a000 = self.sample(int_x as f32, int_y as f32, int_z as f32, seed);
                let a001 = self.sample(int_x as f32, int_y as f32, (int_z + 1) as f32, seed);
                let a010 = self.sample(int_x as f32, (int_y + 1) as f32, int_z as f32, seed);
                let a011 = self.sample(int_x as f32, (int_y + 1) as f32, (int_z + 1) as f32, seed);
                let a100 = self.sample((int_x + 1) as f32, int_y as f32, int_z as f32, seed);
                let a101 = self.sample((int_x + 1) as f32, int_y as f32, (int_z + 1) as f32, seed);
                let a110 = self.sample((int_x + 1) as f32, (int_y + 1) as f32, int_z as f32, seed);
                let a111 = self.sample((int_x + 1) as f32, (int_y + 1) as f32, (int_z + 1) as f32, seed);
                let i00 = self.lerp(a000, a100, u);
                let i01 = self.lerp(a001, a101, u);
                let i10 = self.lerp(a010, a110, u);
                let i11 = self.lerp(a011, a111, u);
                let i0 = self.lerp(i00, i10, v);
                let i1 = self.lerp(i01, i11, v);
                self.lerp(i0, i1, w)
        }
            
        pub fn density(&self, x: f32, y: f32, z: f32, octaves: i32, persistence: f32, lacunarity: f32) -> f32
        {
                let seed: i64 = self.seed;
                let mut total = 0.0;
                let mut frequency = 1.0;
                let mut amplitude = 1.0;
                let mut max_value = 0.0;
                
                for i in 0..octaves
                {
                        total += self.smooth_noise_3d(
                                x * frequency * self.scale + OFFSET,
                                y * frequency * self.scale + OFFSET,
                                z * frequency * self.scale + OFFSET,
                                seed + i as i64 * 10
                        ) * amplitude;
                    
                        max_value += amplitude;
                        amplitude *= persistence;
                        frequency *= lacunarity;
                }
                
                total / max_value
        }
            
        fn lerp(&self, a: f32, b: f32, t: f32) -> f32
        {
                a * (1.0 - t) + b * t
        }
}
