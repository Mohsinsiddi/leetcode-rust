impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let mut rods : Vec<Rod> = Vec::new();
        for i in 0..=9 {
            let rod = Rod {
                id : i,
                has_red: false,
                has_green : false,
                has_blue: false,
            };  
            rods.push(rod);
        }  
        let chars : Vec<char> = rings.chars().collect();
        let mut i =0;
        while i < chars.len()-1 {
            match chars[i] {
                'R' => {
                   rods[Self::get_rod_num(&chars,i)].has_red = true;
                },
                'G' => {
                   rods[Self::get_rod_num(&chars,i)].has_green = true;
                },
                'B' => {
                   rods[Self::get_rod_num(&chars,i)].has_blue = true;
                },
                _ => {
                   unreachable!()
                }
            }
            i+=2;
        }
        let mut ans =0;
        for rod in rods {
            if rod.has_red && rod.has_green && rod.has_blue {
                ans+=1;
            }
        }
        ans
    }
    fn get_rod_num(chars: &Vec<char>, i : usize) -> usize {
            chars[i+1].to_digit(10).unwrap() as usize
    } 
}

#[derive(Debug)]
struct Rod {
    id : usize,
    has_red: bool,
    has_green : bool,
    has_blue: bool,
    
}
