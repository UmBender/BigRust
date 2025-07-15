use std::cmp::Ordering;
use std::fmt;
use std::iter::zip;
use std::ops::{Add, Mul, Sub};
mod sign;
use sign::Sign;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigInt {
    sign: Sign,
    limbs: Vec<u32>,
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.sign < other.sign {
            return Some(Ordering::Less);
        }

        if self.sign > other.sign {
            return Some(Ordering::Greater);
        }

        if Sign::Zero == self.sign && Sign::Zero == other.sign {
            return Some(Ordering::Equal);
        }

        let comparation = self.abs_cmp(other);
        return Some(if self.sign == Sign::Positive {
            comparation?
        } else {
            comparation?.reverse()
        });
    }
}

impl From<u8> for BigInt {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::new(),
            n => Self::from_u128(n as u128, Sign::Positive),
        }
    }
}

impl From<u16> for BigInt {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::new(),
            n => Self::from_u128(n as u128, Sign::Positive),
        }
    }
}

impl From<u32> for BigInt {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::new(),
            n => Self::from_u128(n as u128, Sign::Positive),
        }
    }
}

impl From<u64> for BigInt {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::new(),
            n => Self::from_u128(n as u128, Sign::Positive),
        }
    }
}

impl From<u128> for BigInt {
    fn from(value: u128) -> Self {
        match value {
            0 => Self::new(),
            n => Self::from_u128(n, Sign::Positive),
        }
    }
}

impl From<usize> for BigInt {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::new(),
            n => Self::from_u128(n as u128, Sign::Positive),
        }
    }
}

impl From<i8> for BigInt {
    fn from(value: i8) -> Self {
        match value {
            0 => Self::new(),
            n if n < 0 => Self::from_u128((-n) as u128, Sign::Negative),
            n => Self::from_u128(n as u128, Sign::Positive),
        }
    }
}

impl From<i16> for BigInt {
    fn from(value: i16) -> Self {
        match value {
            0 => Self::new(),
            n if n < 0 => Self::from_u128((-n) as u128, Sign::Negative),
            n => Self::from_u128(n as u128, Sign::Positive),
        }
    }
}

impl From<i32> for BigInt {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::new(),
            n if n < 0 => Self::from_u128((-n) as u128, Sign::Negative),
            n => Self::from_u128(n as u128, Sign::Positive),
        }
    }
}

impl From<i64> for BigInt {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::new(),
            n if n < 0 => Self::from_u128((-n) as u128, Sign::Negative),
            n => Self::from_u128(n as u128, Sign::Positive),
        }
    }
}

impl From<i128> for BigInt {
    fn from(value: i128) -> Self {
        match value {
            0 => Self::new(),
            n if n < 0 => Self::from_u128((-n) as u128, Sign::Negative),
            n => Self::from_u128(n as u128, Sign::Positive),
        }
    }
}

impl From<isize> for BigInt {
    fn from(value: isize) -> Self {
        match value {
            0 => Self::new(),
            n if n < 0 => Self::from_u128((-n) as u128, Sign::Negative),
            n => Self::from_u128(n as u128, Sign::Positive),
        }
    }
}

impl From<&str> for BigInt {
    fn from(value: &str) -> Self {
        Self::from_slice_str(value)
    }
}

impl From<&BigInt> for String {
    fn from(value: &BigInt) -> Self {
        value.to_string()
    }
}

impl From<BigInt> for String {
    fn from(value: BigInt) -> Self {
        value.to_string()
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_owned_string())
    }
}

impl BigInt {
    pub fn new() -> Self {
        Self {
            sign: Sign::Zero,
            limbs: Vec::new(),
        }
    }

    pub fn from_u128(mut n: u128, sign: Sign) -> Self {
        let mask: u128 = u32::max_value() as u128;
        let mut chunk: Vec<u32> = vec![0; 4];

        for i in chunk.iter_mut() {
            *i = (n & mask) as u32;
            n >>= 32;
        }
        Self { sign, limbs: chunk }
    }

    fn normalize(&mut self) {
        while let Some(&0) = self.limbs.last() {
            self.limbs.pop();
        }

        if self.limbs.is_empty() {
            self.sign = Sign::Zero;
        }
    }

    fn div_u32(&mut self, divisor: u32) -> u32 {
        let mut acc: u64 = 0;
        for i in self.limbs.iter_mut().rev() {
            acc <<= 32;
            acc += (i.clone()) as u64;
            let rest = acc % (divisor as u64);
            *i = (acc / divisor as u64) as u32;
            acc = rest;
        }
        self.normalize();
        acc as u32
    }

    fn to_owned_string(&self) -> String {
        if self.sign == Sign::Zero {
            return "0".into();
        }
        let mut cloned_value = self.clone();
        let mut actual = String::new();
        while cloned_value.sign != Sign::Zero {
            let rest = cloned_value.div_u32(10);
            actual += format!("{}", rest).as_str();
        }
        if self.sign == Sign::Negative {
            actual.push_str("-");
        }
        let actual = actual.chars().rev().collect::<String>();
        actual
    }

    fn div2_base10(elements: &mut Vec<u8>) -> u8 {
        let mut rest = 0;
        for i in elements.iter_mut().rev() {
            let actual_value = rest * 10 + *i;
            rest = actual_value % 2;
            *i = actual_value / 2;
        }
        while let Some(&0) = elements.last() {
            elements.pop();
        }
        rest as u8
    }

    fn from_slice_str(elements: &str) -> Self {
        let mut vec_elements = elements
            .chars()
            .rev()
            .map(|x| x.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>();
        let mut digits: Vec<u32> = Vec::new();
        let mut binary_rep: Vec<u8> = Vec::new();
        while !vec_elements.is_empty() {
            binary_rep.push(Self::div2_base10(&mut vec_elements) as u8);
        }
        let mut acc: u32 = 0;
        for i in 0..binary_rep.len() {
            acc += (binary_rep[i] as u32) << (i % 32);
            if i % 32 == 31 {
                digits.push(acc);
                acc = 0;
            }
        }
        digits.push(acc);

        Self {
            sign: Sign::Positive,
            limbs: digits,
        }
    }

    fn abs_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.limbs.len() > other.limbs.len() {
            return Some(Ordering::Greater);
        }
        if self.limbs.len() < other.limbs.len() {
            return Some(Ordering::Less);
        }
        for (i, j) in zip(self.limbs.iter().rev(), other.limbs.iter().rev()) {
            if *i > *j {
                return Some(Ordering::Greater);
            }
            if *i < *j {
                return Some(Ordering::Less);
            }
        }
        Some(Ordering::Equal)
    }

    fn abs_sub(&self, other: &Self, sign: Sign) -> Self {
        assert_eq!(
            self.abs_cmp(other),
            Some(Ordering::Greater),
            "the first value must be grater"
        );
        let mut owned: u32 = 0;
        let mut result: Vec<u32> = vec![];
        for i in 0..self.limbs.len() {
            let first = *self.limbs.get(i).unwrap_or(&0) as u32;
            let second = *other.limbs.get(i).unwrap_or(&0) as u32;
            let mut actual: i64 = first as i64 - second as i64 - owned as i64;
            owned = 0;
            if actual < 0 {
                owned += 1;
                actual += 1i64 << 32;
            }
            result.push(actual as u32);
        }
        Self {
            sign,
            limbs: result,
        }
    }

    fn add_same_sign(self, rhs: Self) -> Self {
        assert_eq!(
            self.sign, rhs.sign,
            "For 'add_same_sign' the signs must be equal"
        );
        let mut rest: u32 = 0;
        let mut actual_sum: Vec<u32> = vec![];
        let max_lenght = self.limbs.len().max(rhs.limbs.len());
        for i in 0..max_lenght {
            let first = *self.limbs.get(i).unwrap_or(&0);
            let second = *rhs.limbs.get(i).unwrap_or(&0);
            let actual_value = (first as u64) + (second as u64) + (rest as u64);
            let cell_value = actual_value % ((u32::max_value() as u64) + 1);
            actual_sum.push(cell_value as u32);
            rest = (actual_value >> 32) as u32;
        }
        if rest != 0 {
            actual_sum.push(rest);
        }
        Self {
            sign: self.sign,
            limbs: actual_sum,
        }
    }

    fn add_diff_sign(self, rhs: Self) -> Self {
        match self.abs_cmp(&rhs).unwrap_or(Ordering::Equal) {
            Ordering::Equal => {
                return Self {
                    sign: Sign::Zero,
                    limbs: vec![],
                };
            }
            Ordering::Less => {
                let actual_sign = rhs.sign.clone();
                return rhs.abs_sub(&self, actual_sign);
            }
            Ordering::Greater => {
                let actual_sign = self.sign.clone();
                return self.abs_sub(&rhs, actual_sign);
            }
        }
    }
}

impl Add for BigInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.sign == Sign::Zero {
            return rhs;
        }
        if rhs.sign == Sign::Zero {
            return self;
        }
        if self.sign == rhs.sign {
            return self.add_same_sign(rhs);
        }
        self.add_diff_sign(rhs)
    }
}

impl Sub for BigInt {
    type Output = Self;
    fn sub(self, mut rhs: Self) -> Self::Output {
        rhs.sign = rhs.sign.reverse();
        Self::add(self, rhs)
    }
}

impl Mul for BigInt {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

// impl std::str::FromStr for BigInt {
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Result::Ok(Self::from_slice_str(s))
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut big: BigInt = i32::max_value().into();
        let mut value: u64 = i32::max_value() as u64;
        for _ in 0..100 {
            for _ in 0..100 {
                big = big + (i32::max_value()).into();
                value += i32::max_value() as u64;
            }
            assert_eq!(big.to_string(), format!("{}", value));
        }

        assert_eq!(big.to_string(), format!("{}", value));
    }

    #[test]
    fn subtraction_pos_pos_small_largest() {
        let big_base: BigInt = i128::max_value().into();
        let removed: BigInt = u128::max_value().into();
        let new_value: BigInt = big_base - removed;
        let expected_value: String = format!("-{}", 1u128 << 127);
        assert_eq!(
            new_value.to_string(),
            expected_value,
            "The values must to be equal"
        );
    }

    #[test]
    fn subtraction_pos_pos_largest_small() {
        let big_base: BigInt = u128::max_value().into();
        let removed: BigInt = i128::max_value().into();
        let new_value: BigInt = big_base - removed;
        let expected_value: String = format!("{}", 1u128 << 127);
        assert_eq!(
            new_value.to_string(),
            expected_value,
            "The values must to be equal"
        );
    }
}

#[cfg(test)]
mod conversion_test {
    use super::*;
    const NUMBERS_STR: &[&str] = &[
        "401430844962386897725262300609361742565971309153334262286471299472153223817183735798789789496276633990371699140425101843642894849943927025984101626154818288395327218426137137356733527590920035665922510928699884461615744656496918571869590365811443360765457530509967569702342527328744717872109053309609721593621377494810470808984727919499290123671445740941972769605782379426918523002523444023806394822651100920205028620432183411695757196138438726590248410014907643092532521125655876870081964384270185541004765526462558939478433737211942550265422250133843343046817878867602377095323834252215848133485484282243151965245110770157103230626824303310910416105943353745122936684509603872737590835535006659608294070665947369457178763038626",
        "1386910106522230273182114737310112804708372277705790166290481875384870317832405203618446039718554024592880061406267805833314031943242636373266657516638442176435941084853304818131050523052511396014662390024551691466201861702522270821681446213358380594921488959964443701107104635279403464004884543643533353132593972168733504749904838137697668934676216733677291235571565191035938798795797317329529878752009647576628022374137110417071412687163112770743118238857035678523290673778197573673409692673885729163885116577148437500",
        "6210167035326326738193870739142198625934696005602689183394171194789917801666876643860418267377046118233854316843612064913069095632027652281793191557755273744052114171292833912037935415296",
        "4386121265860268244651098387443671389331664165805453865887665920",
        "34530751242803838008297397435772949868134089233481117047235447744468991856086891824276194749483049838283024004661607178768641887263191594810047914341904778586518410188492347809294888259711027603230291389182110522741166944857445444295054026147170580785035617941372231093493307886327641316951702496844541990480208321300000000000000000000000000000000000000000000000000000",
        "55838207246645480018282448063359785831254489219656458537975572140987893144169434245913366472828462096916555023710567245730797415478256766164122382178902626037597656250",
        "21176502153106669968715853917875866803113214722074428806562294009200738570123521983975617677693930242829388",
        "82371982754916111926313391442721034227910861148050350342865568485369909459046700786827840405262106010844873215770020963337172612372052453150848656910345250993844147088296749540079004583383289226913994461509015449165432015423465663296809870693996459731260990976058673645499010625736346965885434457769975252598077466842913424924359839517185499848175425480773292712057957052296531582149723502227509105129036531487796541054296883399210844903682157768424154012721418905776304544450993227790651129417890464178353764978772109957201233542454534364613507421387491236013291562670328435388797262297818902421169368544572933750843770997681143290886005249222005729225879238328794756383881349507547283871975148698708816268233145403310080",
        "1009329238090374301530827626112017579052076787230723728163943419777177761212404696568108030238818506163042140971139964286095126668133869287651724825239685792886745655015364923272748674499968447867621710201608900596481197012290800561930922602091767530153358700639988005028345053403734128736433650877538697357418604504962991959400448660890635503690566228506812044380631088602834510595429851146186129267652635646122356680600524770417365840760997512795274745554574218579920938387850526396505301818251609802246093750",
        "140391568331696610534803930167674088190349914142198509838225452036402778158289487909559070293800010084754604615214583763593222575677661243281352934830652229841173905783188172497564136267040418508508158112602256030102194426934506583179673337355117684703594491502122295851423076005011",
        "1353096104151102199233494025473108465974304542185506182082028545388748153199036202696673953673968612100716994560",
    ];

    #[test]
    fn test_conversion_from_string() {
        for number in NUMBERS_STR.iter() {
            let big: BigInt = (*number).into();
            assert_eq!(String::from(&big), *number);
        }
    }
}
