impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut vert_pos = 0;
        let mut hori_pos = 0;
        for _mov in moves.chars().collect::<Vec<char>>() {
            match _mov {
                'U' => {
                   vert_pos +=1;
                },
                'D' => {
                   vert_pos -=1;
                },
                'R' => {
                   hori_pos+=1;
                },
                'L' => {
                   hori_pos-=1;
                },
                _ => ()
            }
        }
        hori_pos == 0 && vert_pos == 0
    }
}
