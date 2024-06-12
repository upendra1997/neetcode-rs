use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut hashmap = ('A'..='Z')
            .zip(std::iter::repeat(0))
            .collect::<HashMap<char, usize>>();
        let mut start = 0;
        let mut maxi = 0;
        let s = s.chars().collect::<Vec<char>>();
        for (idx, i) in s.iter().enumerate() {
            *hashmap.get_mut(i).unwrap() += 1;
            loop {
                let max_item = hashmap.iter().map(|(k, v)| (v, k)).max().unwrap();
                let total_count: usize = hashmap.iter().map(|(_k, v)| v).sum();
                let extra = total_count - max_item.0;
                let extra_to_be_removed = extra as i32 - k;
                if extra_to_be_removed <= 0 {
                    break;
                }
                println!(
                    "removing {}, because max_item: {}:{}, total_count: {}",
                    extra_to_be_removed, max_item.1, max_item.0, total_count
                );
                for _j in 0..extra_to_be_removed {
                    *hashmap.get_mut(&s[start]).unwrap() -= 1;
                    start += 1;
                }
            }
            println!("window is: {:?}", &s[start..(idx + 1)]);
            maxi = maxi.max(idx - start + 1);
        }
        maxi as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "ABAB";
        let k = 2;
        let result = 4;
        assert_eq!(result, Solution::character_replacement(s.to_string(), k));
    }
    #[test]
    fn it_works2() {
        let s = "AABABBA";
        let k = 1;
        let result = 4;
        assert_eq!(result, Solution::character_replacement(s.to_string(), k));
    }
    #[test]
    fn it_works3() {
        let s = "KRSCDCSONAJNHLBMDQGIFCPEKPOHQIHLTDIQGEKLRLCQNBOHNDQGHJPNDQPERNFSSSRDEQLFPCCCARFMDLHADJADAGNNSBNCJQOF";
        let k = 4;
        let result = 7;
        assert_eq!(result, Solution::character_replacement(s.to_string(), k));
    }

    #[test]
    fn it_works4() {
        let s = "CUTQSTZSZBMBCVMQLHTHCEQCICXEKJPYEPKLUJRCUULAZRAVPVKNYQIMMYTTMRCZVSXWNFUWXEOVWQMMKMWVFRBKTVISHXUYFXYKIJUCFUMMJZMPAFDDHBALJZGWQMSDXSLYLJHDHSQXSVEEYKFQMMPRRIESRHVJBLAVLYWMTWUTMULMKRNGYBTXBLEGLTWCFEGGIQMJCXEWNKCKBLOVQOXAKBUJIWXYNVCNVGRJKPCSJPZVPUXKVVEJPUWEQPBQKGJBKRKEIQWNGHWDSRUXURWCTMRUFBOWDNPJWYKPKGUXECVQNVKNGDFRSCGJKYVCRAMHHUPTGITNPIDORQIARYILWKBKUVUBUOZFSKBUWLGOWCNMCILCMLHEFPQBMRYTMIZONXRBKXPUGAMMWXDMKDCAWOXRVQNUEAPXOFWFNSVTQCWMDFALZKOFIMKQDICYFWLDILBSPGXTELZROWSOBHNDZMOBJHFUTPUZLVYOUECDWKBZDJTJIPJHAHOFECLMPGHJSDFMENLRAJMWAQQOTTAGYQFRKNMPJVAFUPFRSZGKHCAGGBQCMCCFVOPYQYYCAQIOUBEKODXUAASRLGCHFVWURZHLSZZSISJGHSEEKKTJETSYSXTRHUAQGTLRYGQPVHCKKUUDNHNWRPOGWDSQFBRVAAPEUYFTBBDNHABKOVVLCGMTFIENQWFSWHIQDDTRBLJBYBRHYEQWETUGJWWQNJYWIBWBNWUSPFQKFFDNRZXPZALQNDXIOJWYUIKGWKUHYPMMZOIJSEBOJFOXYVQRZIDNXCZWEDVFAYYEXDYGUHCSCANKJRELKPWWHPMBBOZNJDDZAHMTDYZNVHMAYZVBRDSSIFYOTKUZXGFVZMURMPANIWSLNKYXSYCAKFFKAZPTGADMVSAPRCPMEJKLNWBUKAFKVFEMTCWIRAKSCMKNLTCFUIUTZDOYRIDXHOTQEZAERTFWXVNKSCTVGKAXRTWLKXYSMEWRRVYLHEGLWSWBRGBASLRRTYYHKXTMAKGZKPSZEOCMFWFNULIFJWXOUMXPPCVRRIYLEURKJTVQIKFYCGPLTDCFINDZSNIGFZQGOWDLRTCCYEHKXINCUYIPXQEPABLMGVNHZHOSWOCYPUJIJMVSWFYSOFRITVKSKDRHCNSTLCWZLRLYNDJVPANHXDWYKLGANHAVFDGMIZVARBDGMMESZZUIUTAYOMLKBMAIJDWVAVRURUEOHXWZGLJFWVUCPBNNDMDBGYJBQPPTHITPHWKHTUVDRJMCQDPRXXHYNQXDVYQRPSHRBZCYTUWYTFRCWIBDDOBDCNWGXUEPSEZCEVZLYHXAAOGUEILKSVWHAFLAAKXAWVBFHLJYNXNRCKHAJCLTZYPBJPFABUBQTBYXHGUZHWZJJSHWAXDOMIQZWUUXYYZQVGNIQPREBELTWDUQJJUEKIAJTUAZDXBDEHSRQYXHOXYEXGKYDMBJKJIMPEEFQRRARUDEPANMEROXGHTVKTOJFZDTGZMJLEPIXSKCWYEMERBWIDMIKFXRJUDVNRNSOCXEDAUFXFRZRTZHTKWCQHSNBRGBKHUSYIYLEAIOCIUMWRYLQXAFXRRJKLEUNQXQUOGGDQRCGKBWHSJHWYUYSWUGKIHCGPNBCOTHWIDCZOVVTBUURUSZTCJRAWTNFEQMLVBPVXSTUGZXCBKAZCCPLPGKSHUUGRGWQAAWURSUXPMZCZBPDJZVGVZPOCXOFYWMMFOCQUQGHBMHIZXEJFXXTNSNCJYHJXLLAEQFKCMQLQTIVESDRDOMJIPXXAIGXEULGRZZAMUNBHBVYJGKCLVBFTCAOBFUXOHOECQWBIGYVJGJISMPNXMKURJSMKOLUGTJJPLTZTRRVGSLWCBPOXFTJQXJJWSDMUSOZXIABESOWKYHHORVNIEOVGPDMHFSWWVGGVPSWGHDQHLDBRQNEKJXVNJKPUYAFGVJLBXADKSJWHJYEOPWDVDZWCUXBMMKAWKGJZZEMDODAWDLHHUOSDQCDCXCYGAOIDROWMMVSPNCMRBBWFMXGJEETLXIHJJSUURIRRMPGRPYINRIJGKSHKPWEZDALQEVJIGICMONLBXIPHQFJPMFMCYMWFADFKJNLZMMHCINCOGBBVKITFHENTZZQTDNWGDOPJUNHJZEJIOZXFBDPIPTDYFCUQDBCPSNUKKLVFIZTUZWPDBGXCHUOACWEBUQQNMYNCRNVKJUEDUJKYXOHWSFQWGBQHPTOWRITQALXROVQZWIOILGDXEMKZIMGRHDLGNFUMOWWQJVQIOKRRPRVULSSYGJVGTVWYNKCCYQJNAMXGVRMMCPULLSIDPQQMGMFSQWFBPTQFCSZXZBZQLZEEAQBYLFWQHKVWAWUWUYMFYQLVBWNBYFEXOFTUMOVDJYEWYORUWQWDFLNGMNHIPJLFTRFAGFRZHCOZTYHDFFRIKYRKCJOPWPUSDWLRUTKWTXTMECNBXQAVGQTXQHNYHUDMAUSYJUMAAPJUMDIJDAYBWYFHOKVWZQYWYYEQBOMARSFHEZZDYYMONCVIDGFIRDJJLTNCTPIAXJMKMBQSXNPYZPURGWYSHXWVZLOBKWNINJJYOHKWMGKNFQBQIPISJDIOJPQBXTVFEMIRSZAPEITGIFEJITKPSMFAEJFWRJCWOXMKDPVVCDPQWFTRCIOYBWALWRRWWGKXURBDRGVUCKIFTJODJQDKHCOHOFLSNLFAMWBXYZASGCBFZFGFRQVJGRDTKJKTABPPYPHWZKWRLCFKJDEABOSIJNVBBELNBMYJUKZSJBNHHXUCMAXGLYSJDBGJCHTYKJQCVPPJVNSCHRFRUEZOHGKVQLVYKENZHPIODXFTJUVRFCLQPPGAFNMZJCLGDTZCWRVNERBASFLEAJFEYGIRLFWAFSLSUXVQJUOSSYHEVVHZYMRQFDRVQVMGABPTFSFJDGGYZRGHWAQTJCYNRHDGOOCTVGHLUOZWVOJBLSQPDSQBBRGQADDQQKBOFBRUADIYPPOYPVDWUBRTUTPTMDLSFDUVWPEECKFHYXDUPVLNPPNKBSYYOFFQKEJKJMBUQYJSRUJHITJOQGMSBIHFXQOQIFQRLQDDCSOSPUOEZBLJSJZNEWNFOAOKHEEKXDCYITHYSXOYMEMNZOCAZBGEFZCWAPIZXYDKSUJHTQPGPHYPDOWEBNODTLLMYOGGUSNKKKFVMVAGDPZRJKJRRQABJZUQKOUSBIGBJAEYFYWZMPNAHQJPZVUNVUACTHACPECEBJMVVIRXHHDBUPQUWRXICTYHLSBVRCLFPMLQVLQJDDZPKZYNLQXZZIETGYBURDFXPDAABRWDCCOTYPVPFUWNRGAGLXIZLEEHSNPIVSNRSFVFRIGIKOMLUMQEVJKEEEQUAQTRSJDAXPFYMDVNOLSICQDTYKBYPQYODWREMAUZZAPHPOCBAJRTELEYGKCANGNTMKTOODJMDOHLLYDILSFMPJNCBKXSCOEQJREWSRZWIKJUIAEIIKWCDAWRDRFLXQVYMIZLKDKSSIJGJZYOKIZXWOFNHKMVUTZALRZMBSKADAZTBSRRCWJEFPBDZMWRMNERMQPMUKPBJXQKAHJOBBVVMEHAOYHLZMQHOXVIGSRMDWCNBATUSVMBFIDIYWEBWKLKWOVJROPXNDRWKWKDDQENSAGTQKGQBJUWSUKBOIJNVBJGABTOSSMVGQFAPCRGVGUSBRCUVJEMCVUFPKMEXUDSVJBEDGUVNMNBJKEVWVFFSHWOVPNUBWLZYVREWRRVGVYFMXEUIRXPNEIDEEVHJLVHSXGNYWLOCRWQUUPRZNXQEECDSYCTKFEZYXCLUBVTDSWKCFRTHKDKNORMMQNPNSZVSKSCOQTJTSGCCSHAXFLRYSBWCLTZAHCOUEAJGOHRTBVIXSQFECEVPEOKRTXFPGCTVDKDQJIPMLOGFAWGXKWRSSGNLVQIAOKJFXQGVPEIMIKKGVZGKYQXYKAAZLTJDEJXYGVCXPLYVTQQIAZBABDPAKZODIASBJJEGVOJBMKEMQEVWFBKFLZXVNGWFSTGBQMHBRSFJFENSKZJUBQFLWEMBBUQLYFYRMQCMJXPZVULTZALBZHVLCTPRBKVHRGVJDKPRUAVOEHQJCIMWSYIXNTYYTGBAPZJFODZKDYERJSBUPAPPHBNSLYKKAFNQPNZSRHSXCOQPSNWYMXCGFPPGWAPNATEBEDSJFDTZYMKEXNTGJJOZKACFNFVSEECSIOBCMINUIWNLUDSZQRFADMJGWGAOZGWPYXARPYYHPWZCESKNROBWBCSXAMMHHYPWELCPPRPQIFCVNGVXDQHULUPRRQEOMSGESICSRYEHCWECEORJFNUQVJMAOTSJNBWKTMHMKDTAVWFHZYZZIOAIKRVPITZPYSUWDVRLAUPALXXQVYTJTVEFBETLBSZBWAUZVIYUMUVTIMUZQBLMPNVSFWVMTLIFYAHSRIVICTNDZZOZHAXFPSYGHBRYDXAUZEGFMEHNZEJGJDIBHXPVYENHJOHOHDWQVGVJJDGBSITVCZYPLKXZIBHBIQPZNDDUTBYSYKNFYVBWFENYUAAVJDWHMZHIQUBZQBYIZYFKYJSKKLUSFFLMDUIHWOBZQBAZGAIFHPMORCYKUINDJOIKKKCENXJSHZMYOEGLTVBHRJPXYGHUWJSXXMYJSETKWDEDARIQPLWHZXTUGEDSCHSNMOGMZASXNEODTQVSLTVSVQBJWXCSGTCUIAOLGEKSEEFBCFVOPIKOVJONZNRMGZZZAXFOPPLICTFTNSSEFWABMXQPRIGBBUBWSXWTNQMZRYRHROHECVLVBDTLDZWTOWJUCDKPUOZNHTFOYJIALSZDCCNXTUEKXEZRWVLWRJYWCYONNZVLTOGXKWMUYJTGEVRROPFLETSGLWRUWDLPYJAXOTWMKVBPDNESLBOYOSSSFVARDTEXNHCZTVBJRQWWZJDYZKPTZCBPAXSQYUDEKWZVCHDNHUPKSOLWSHDCHXYFVEBDTXISBUCFWPCKXDHUCLTOPFJOSCCOWHYJGRNDWASIFKYECWWBTILDQJTWXNRTKNBPISULXMOFDXVNVFVLIMVHTTSFXUQCPKJPZIJUASYITFIMKVQDZBTYJBFHMYHRHYRMZWUXRBDCLTLZEUGHGNAZHVFOTEOOOKMBSTIPNATUFDWMPLGQSGPEXOJUIFZKOMHCXADOOPCZAFTIDDMLEFSFDMPGFHKNITQNZDQDEUHPYQSBJTXJFIAOONGAQMTRQDTKGWPIXSESRVPFHRMHNHNSNOIIHOOHBYRLYVBHWHSDMYZVNLVKLEEDTGQTTXDYANXZKXJZRNSTUPOCDWJILNVVDDSCTJSZJKDCEUJRETFTZBBDKXQNNLSXIRVAVSCRBQBQWBVIWPEPTPBTZOYWJNDUUVNYAMZRCKOBAKVPEPDGSMRDQCZEZEHPOXTKGMBOBXDRYJLBGIMFFVNLCLRYWUEFTBHOKZXJANRFUOBNTYSNLTKQSFDTNDOAPLTKSZBSXFJWAUNVPXYIAXXBOCLVTALHYARTBPFSUPILRNNBXFTHFJCRNANYEWYYCZGITRPYSPBPYXSNMZBMXUGWIIBNBUCKMQCCHKLZVPRMPPKMXSBKQVQSJSDGHWVFSJHLCSMQAARRPOPITAHIPJPJITNRQSICWTBVKJCZSVBJWPQJPGELEMLNSHNQIEXIMRQAKYVVNIFSIEIXHCJUBZZPCOLPWGFTHFCUTWBSWDHDUSPHZCUQIMOEXICIOEBRRFPMODZLQNEKEKFBPJTTZHMOMVOAKVYOMBOGWOIVVPUCKHSOEFKDIRQLPRHWIKXRFWXJZWNIJZXPTJARTJQTGEEYSIDKIGXMNQNPPFIJNESULETTHMXJWDOEOBWVGNSXLLBHIGFWKXBZPLQOAZDMVGKEPGREVSNRAVJBXCBWFVPZLDTNXRFCOEMOBOJEEPSABSCOTEVXVFWZNQDVTFYFZCLDIFTJKYBUAFNQJEMPFBLTJGOEFOPQACKREKFCCOOEKRVDQZGPPROJOTJOONCIYKSUOJWZULHYWPMJDPCRXRIKCAAOWWKDARYGVPGZFZBXKZOCQVHUQQAQGKTTDKPAIEYKYETDDJAUQDIPPZKFOLGKNISJUVBMSATGVVZXOMYKFTPTWASRETFOBISTNJWSTFGZXURAUDOOORMQOUNXBFDFWFUDFYMOWUAGZPKZPWCMKBUKORLYZXAMTIRYATQXXXAESNGICMYMXVUWNSUSDGHHZAXJQPTCBVUBWSBUCIXUTJUJIRDRFETQKMMREECNIPGGZHUXWAZAUTFTDXZIKCUCFVXLFBZOOVRYWEFBGQUPJDSDLPLESIBXGZHZFZKXFFEKTJMAJAZSAJXAVBERMTPEFFNRFOCEVPPUVNRIXQQQTCSVQNUWCKHEZBGYJXKMYJCIUTNIXVCAIULWIOUDSHZXZUGJDGNUQBYQPEUEGZISLMPEJJBFBREEXJKZEPEVJSQGMWCJDEAXXPUJJEMMYUPPEKARWYDMFXJXNSZLBKHOCQPIZWVDLWQSLQFAHWSIQQSBJWOCEXDOJBYIXMTLIPWQUXREGJOCSBEOCJETKQSXLGINLDEHHWXKNMEGSDEWGVHRAINVVTYNCOPKHLYPDZWQQQCXGUQXISWMEZNUYTLDFWEHCSCMYODRMAPJAUJSBXUYHAGWIULXKSXYRUYTLEOAJNBPEWIMWPQXUFUGKBWFAKRFRSVXGTZBVZIBDWODYTLGKJMSXHSFKHELYFAJAOPYSNFBQAVELIRWUSGNGVVWYBUWKWRJUNCRCQZJQSJUTKQLLXMWUOHLURQRWKDUVGFMNCDTCJOTNFLEVQWSKKHIILKGUBAWCNJFGSLMTGQADDNZENJQBTYRTVXCJYTFMNXCNUGBTKWOZMLBAJEBCEHOZSSUSYWEGVLDJPHPNEHQDUPRRKIGWJLXBJALUYQQPVTEVREDIHLBXPWWPLZIIPWDBNAIUACHLTPFRUBERZGBRZKSNDFOMFDCHQXEUWRWVHYIRQKDSCCQWMZNEBQSBKHNNYHXSDWPPPSVYAFFSWIRDBEIGNBDILUWMTXAIAAQOUZSSQYUQEGQCDEXPNKDWOSXBFRWQHOXLPNPPQMQFBQQNKGRSLHJJQOECCIRTEGTHIDNZOSEMSUHZCBQGUNUOVJSILDUUIRFONLSWUJDMSZYSPZITJTIDOXEXVHUCVULHDYEGSIZWQFLZVWDNCEXZRDZILLKBUQHUFXIVLAPYGYFTUNAVCUYEHWZWJHQXWWLRPERVMJIQADVNMASOYPQSXITDPCWMEFEJGZUKCCYMPIWHGXVUUQZCAZLXIYIHJUAVTLACTSYESSDFWHDYDHYAPETWKTZVQLOVFYMMCKPGQZGMRFSALGJXXUXLTMOQSYQBAKBJMLXVKTXZXHXYXTRAKRDIONDBZVLCDKBQMPBIVXLXXKNDPTCZFNLINXJIJZZKBNBTMTDNIULDLHSAQSPXZKXDTBKVVCYZKGFNKPPGCWVWLNPJRJKAJAPLJDCVPOHVRUCBMLWGIAJTWBUOZGDRBPCLUWFIWDGMNWONHDISLCUHYYYLGHGQEORYEFGYFRHQBKRKFPICJNMEWTEATTSTYOYLQHLNALECIZMWNBCQMVBWFSPSYCZPVZYXPDNYQWVFRVAWWHTYHWEPIKDHJCUCAREYZKERRCXQKVUKPWGFUJPYQAEFXYQLMCTAGCZHDOBJREYXQCXIBPRUMVJNPCJDBMPQQQUDCENPVLMYJPBBXZRLEJXECCIIWHONWQPDKYRWCKLFMVLPFXCSNERJLOEWHUGFEOGNZWBHAXRKDGGVTQBWGYEIFRJQBFNKGSQECQJVEZXOZRDJNTCYYEOFPSEYKJAMJTXQUFRARYDGZCNRYSIUNGBZXRAZOXFZVMEEEQJYLGWMYRIRVZDFOHZMPQPYRQNHLEPVLHXNUNWKAHSOWLATKJCDZDPEWUEWTKQMAZSJPVOQCUBJRERXRRJOGWHVZJUJXXJZMLCSLPEPMBGGULNYGCTPPDSERBNUAAZRKUYXRCIWLQZDREJEPKPPAVLOQDJNXCAIQWDAVCMOQRHOWTRPHONBVRMIQAZKLPEPDYCZAGCWETPIECCITHJGDWOHYCMJLJVXFBXJNWSOAOXBWBCAIOLGHAGNWOEEOMXHDVWVSFKFZTQISTBFXVCERUAOEUQZMGBJMODTZPTNHKGYYRPGRAMNDMACMLBKZDHMVNKKGCSJVMQOGDOLCWYYDKWWCBPPVRSVEKDUUDOIVWWFMBIAUDGDIWOGAMXVOOSQOOVAOVNUYNYSLNYRIVBBTBMPUXUCORMRMBYVDDGTPOWZRFYLVRJDWYMGDYFPTPEJSUFNITSPSBCQZHTCUSMZFDTSSNBGHPQHRWCRMATJIPJCRPVGIOCWFNVOCYYHEUQEADNUHRWNPDEKWPOHJGLLWNJKCWJJGHXROZSEWQGRYTIFANKGCQNTUJDKTIMDIPRYCVJEYHAHNLBXKXYMNBLUNUNPSXQCHWKFBBPSQCJNPZGHGEQJICGDMGQOMCXQXFGXACNPXCYBTJTVJYJEQAYCGXSHUONCNGIYWOFIBFEWWECBLEVYQMXZEALYMIFLSIZCBKOJAOSFKESVEEFWMMWMKCDZVGOLGGSUFLHEVFJBSEOQTORDWCUNKYMOUMFOKKVZTWLTAYCCFXEYYGXDIHFVIUKYJTBNZUYDOPJGEZMUPCPJXLMOSXTBGCVMMCUOUKCZQEKUNOQPCDLZJSLBOENWKDCYKFWYNOCTFQAGXRLCJJFFROJUGGLZKIBGXHIAMIYRBWVUYSSJCMFVBQSHJOMXOFCMTTDIUPYRMGDRACBPQZCHXKZOWUOQRWZTGDEBILBMNGEAJWRXMUDRUODXGSUCCNMUPYWABYZZBUZOPRGUXYFBZYHIQLOCTRANAQGCRFATDBADZMSYFJHCMZJEJZLXLIZVQTDLSJNIOXNTXNILOSEEFVQQZYGQUGZCUNHEEEPXYPHQJUHHMKUAMYHXWROSTAEYSHGXIJKAOXIESLOBICIRZMVAOKEFTJRXGOOHHGRFEWVXMSEHPTPUKUQZCOWFQFVAIOAITOVYQSZNRLTZSNOKCWIDNWMICTQUGJDJXMUJLNIGVBPCVAROSANVUPWWHCFVDSUVVFDBXQBVWBALEGBQOGIKKIHNWWUTDHBGEZUMEVHBTRSAKAYGYOSFEKPZRUBZQWATYHMSDBOCUZTMJEULVROFNNHXAEOGKHVFNEUISWISTUDLAOWEFXIIFEIXPWSJZAKXPSJFCJTBCDVSTAGGCAKECUQPIMCDXBGUAATNDTKDVVQPLMQQGOCWSEDNNACJCHLGDJOJZUANACMTURVPONNOHRIOSBGCKHTYNVCVBVNXLDOLOFFIGVKXQVUNHDBUGPNTBFODMHHNDUGPTRFSZPFRQUDMUHVRKXDTZNDHJLCMMDKOXROGQKQPWLSCVZMUHOLNOWFBZYAPBMCDXAZEMJNWLUFNJOZQHWHSEMZEUQZBEJTDGERQGTACWPWWIDKJMBFDZCYTCZLDDGVPVJAFAJODLFJQMKTNCHOZFEZJUFIQMDKUZKABARZTGYAJDUMKLCOCKOAGLIEHHRQSTITNCYTCXMNHZJWEFPJDROKIQULNMMMLTJNCOCHSTTOHORDAALCSDRNRWFSVAKHLBVJTTXVBDKRWYFMJNYQQKIBAMLITRKMHEZNFBIWDWVBOHSRODSLOBCFCGXHDCEGMOTAFOFWUTIOFIKJLDQKFBXDLNZWCMJTFEZONCKOURMZURYSQWYXWMSNXSXYBRYYKBHFUENSZGYEICSJNXSMBOOJAHWXRCKCRKOPRZWNXMUMEVBNJYKDAHPSTUUNCQCHRUYWZULAWNSZMGZCNZIGHPBXSOEGSALRIYXFPFFGBCIZUCMFZUEFGAYQKBQJRQHJJHCGKHFCBNBRAISTMUHSKQSXBDBLAFZRTVOSKLGLBKOLMYLZTEDRZOARTZVVSWRVXSVMZXGIAEDDDKPSPPENZGDIK";
        let k = 100;
        let result = 115;
        assert_eq!(result, Solution::character_replacement(s.to_string(), k));
    }
}
