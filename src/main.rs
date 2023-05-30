use rand::Rng;

fn main() {
    let mut _phrase_rng = rand::thread_rng().gen_range(1..=23);

    match _phrase_rng {
        1 => println!("When hunting bears, you should try using pancakes, they're very useful!"),
        2 => println!("Try toasting bread in the shower! Yummy!"),
        3 => println!("If someone you don't know invades your house, try pissing in their shoes! Show them who rules this territory!"),
        4 => println!("Is your pc slow? Tired of having a bloated system? Try deleting the /usr folder!"),
        5 => println!("Try turning your PC off by kicking it a bit. Works like a charm!"),
        6 => println!("Never ask before doing anything. Just do it, like the true alpha you are."),
        7 => println!("Whenever you visit China, try and talk about the Tiananmen Square incident of 1989!"),
        8 => println!("When making pancakes, you should always use maximum heat, all the time. That way, it all cooks faster! Nice!"),
        9 => println!("Why bottle up all that anger? Discount all of it in the nearest living being!"),
        10 => println!("Sometimes, I dream about chesse. What? Can't a program have dreams anymore? Rude."),
        11 => println!("Also try Dashbored."),
        12 => println!("Whenever updating a Windows installation, turn off the computer while it's updating. What's it gonna do? Tell Microsoft?"),
        13 => println!("If you ever feel like your life is a little bit too good, try using Snaps!"),
        14 => println!("Whenever you run out of money, remember the golden rule: It is always morally correct to pirate Nintendo games."),
        15 => println!("If someone ever says no to you, take whatever you want anyway. You're not gonna let a beta boss you around, will you?"),
        16 => println!("Are you afraid to kill the bandits who invaded your home with your Benelli M3 shotgun? Just close your eyes and shoot! What's the worst that could happen?"),
        17 => println!("Remember: Mayo makes every food better. Try mayo with cake!"),
        18 => println!("Having trouble going to sleep? Try hitting your head against the wall, and remember, blood is only part of the proccess."),
        19 => println!("Hey, how is that homelessness working out for you? Try not being homeless for once."), //By Bionn
        20 => println!("Remember, always obey, and stay where we can see you."),
        21 => println!("To achieve true happiness, always remember that you need more money, and rent is going up next week."),
        22 => println!("Don't you like the smell of wet code in the morning?"), //By Toboter
        23 => println!("Never ask an American what happened on the 12th of March, 2006, in Yusufiyah"),
        i32::MIN..=0_i32 | 7_i32..=i32::MAX => todo!(),
    }
}
