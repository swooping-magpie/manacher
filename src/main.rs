#![feature(iter_intersperse)]

fn manachers_longest_palindrome_substring(s: &str) -> usize {
    #[derive(Debug, Clone, PartialEq)]
    enum CharToken {
        Divider,
        RealChar(char),
    }

    let num_radii = (2 * s.len()) + 1;

    let v_with_bogus: Vec<CharToken> = std::iter::once(CharToken::Divider)
        .chain(
            s.chars()
                .map(CharToken::RealChar)
                .intersperse(CharToken::Divider),
        )
        .chain(std::iter::once(CharToken::Divider))
        .collect::<Vec<_>>();

    println!(
        "the size of vector is {}, the num_radii value is {}",
        v_with_bogus.len(),
        num_radii
    );

    assert!(v_with_bogus.len() == num_radii);

    let mut palindrome_radii = vec![0; num_radii];

    let mut centre = 0;
    let mut radius = 0;

    while (centre < num_radii) {
        // precondition:
        // radius is set to lower bound for the longest radius

        // find the longest palindome starting at starting at [center + radius, centre + radius]

        while ((centre - radius) > 0
            && (centre + radius) + 1 < num_radii
            && v_with_bogus[(centre - radius) - 1] == v_with_bogus[(centre + radius) + 1])
        {
            radius += 1;
        }

        palindrome_radii[centre] = radius;

        //
        // centre is incremented and then we use precomputed values
        // this is where the magic happens
        //

        let old_centre = centre;
        let old_radius = radius;

        centre += 1;
        radius = 0;

        while (centre <= old_centre + old_radius) {
            //
            //
            //
            let mirror_centre = old_centre - (centre - old_centre);
            let max_mirror_radius = old_radius - (centre - old_centre);

            if (palindrome_radii[mirror_centre] < max_mirror_radius) {
                palindrome_radii[centre] = palindrome_radii[mirror_centre];
                centre += 1;
                continue;
            }

            if (palindrome_radii[mirror_centre] > max_mirror_radius) {
                palindrome_radii[centre] = max_mirror_radius;
                centre += 1;
                continue;
            }

            // other option is that palindrome_radii[mirror_centre] == max_mirror_radius
            radius = max_mirror_radius;
            break;
        }
    }

    let biggest_in_v = palindrome_radii.into_iter().max().unwrap();

    return biggest_in_v;
}

fn main() {
    //let input = "fdsfgdsgsabcdefedcbaerqrreg";
    let input = "ertretertacdaaaaaaaaaaadcbrertgretert";
    let output = manachers_longest_palindrome_substring(input);
    println!(
        "the input string is {}, the output string is {}",
        input, output
    );
}
