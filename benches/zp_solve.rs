#![feature(test)]
extern crate test;

extern crate ndarray;

extern crate reform;

use test::Bencher;

use ndarray::{Array1, Array2, arr1, arr2};

use reform::poly::raw::zp;

#[bench]
fn zp_solve8x8(b: &mut Bencher) {
    let a: Array2<u64> = arr2(&[
        [
            12014962059279119808,
            11729274069130469628,
            14024853910770558621,
            10794592809739201699,
            4591412351669281830,
            4443533779034349939,
            5122371541606635158,
            4428046401013815517,
        ],
        [
            17360236418348383593,
            7136602285067754769,
            277975438643526457,
            13988366963637062310,
            11965422247792341277,
            13473116308090246711,
            9720315817616706606,
            2636590053094494344,
        ],
        [
            7108548395280314459,
            12447903413556513007,
            16516095244241258659,
            17495513749915422285,
            16188271785371370840,
            18135609589437029893,
            11711259525823976471,
            16680947892577840726,
        ],
        [
            1129304879239752775,
            18139328041392429210,
            6590139733972204606,
            16842393362184162587,
            7081222189249445142,
            9670603388459145355,
            9170128611777118820,
            5429443692103467539,
        ],
        [
            4815045248221734879,
            3014716148816528348,
            15231114635082323199,
            7413374795010683548,
            12084545366617579036,
            9103619689315494462,
            15845977628716689791,
            12049087827827906729,
        ],
        [
            11001734271556191863,
            5792364747427335632,
            17026075997810771139,
            3998225937108019800,
            1779015145730982444,
            4042960271256094210,
            6905577323187467150,
            4558133560710470099,
        ],
        [
            9470362143049275666,
            15050630567399234658,
            6477290068444969760,
            1950689046353772704,
            18337880567558499287,
            16153744492842911000,
            10420312895212917543,
            7243988142695285055,
        ],
        [
            12411888002945832209,
            8217010074140802182,
            2276794339879158233,
            7998145881836688847,
            1198631103694420174,
            3520712409374091016,
            9366100562799842596,
            5883351156761682750,
        ],
    ]);
    let bb: Array1<u64> = arr1(&[
        5967034585494953246,
        1733514736420351156,
        2401463918407195129,
        3154881803474453214,
        16143176517113548940,
        3423508916817192534,
        18114954619118863327,
        14281169057397946455,
    ]);
    let p: u64 = 18446744073709551557;
    let x: Array1<u64> = arr1(&[
        3824785585352974503,
        18302955554254014472,
        16542358835553968996,
        2823049759788259690,
        2318543310909473196,
        13688369703899785013,
        16173263261885357154,
        13705729413267355427,
    ]);
    b.iter(|| {
        let r = zp::solve(&a, &bb, p).unwrap();
        assert_eq!(r, x);
        r
    });
}

#[bench]
fn zp_solve16x16(b: &mut Bencher) {
    let a: Array2<u64> = arr2(&[
        [
            2716266586725149706,
            1002152168051751201,
            15321153411862608064,
            1699002711712711355,
            9866780219866807963,
            7774879481971804455,
            4249302022483019984,
            15211735995855867663,
            15941876056185115092,
            6488806519442418286,
            1393756248746427524,
            6413237516676654729,
            9826439573912690928,
            15112231685801503200,
            10910920433729604000,
            2224528520028731218,
        ],
        [
            9028366816538320718,
            1035038573151821347,
            14959158726825061024,
            900504911948864023,
            16672958546365893140,
            10886716909735720826,
            16359108060865798650,
            8413898101485732991,
            2554071548663938105,
            2251553460669484450,
            5889872255898183197,
            4479290932022909468,
            5666694618689871008,
            4098146542413159010,
            5517252510596937576,
            17600669528308725987,
        ],
        [
            1902416916758332124,
            354078651074126646,
            14279364473627172088,
            17796742787041196562,
            16664622720743559269,
            13671323185448146809,
            1573441406005610678,
            13549150012229164088,
            2840604448176095287,
            17438579645014962093,
            10660875040325556785,
            17385146286906599232,
            2178731225286818658,
            13768530645172893119,
            3442332345903384581,
            1904764345747104968,
        ],
        [
            11507605787571646693,
            3829002822890469354,
            4011025829191054153,
            11956485523750252279,
            576697912497422821,
            11603913297168726032,
            10509141683418320688,
            7039523805833571605,
            5997924543719635490,
            3691095481879912577,
            15017327516705715354,
            16329819762480881945,
            5504768720545002310,
            9728136808027400494,
            4665890133086688488,
            3745378522780557607,
        ],
        [
            9051663644407822275,
            6472882059240763723,
            16113651000636438176,
            14690710262461989522,
            14463582170013274069,
            12709162668137415446,
            11608844924478819452,
            7522583121751573546,
            13383835656753923941,
            15108804167976681560,
            12643647479753573128,
            10305625691074421399,
            10644340002672729466,
            6657674405965257323,
            9567832887475927448,
            10104424061586965821,
        ],
        [
            6935658869152936975,
            17497771532978489740,
            3931409304235341604,
            15585513782347390615,
            11926437396747845701,
            12812728197318597730,
            7337929603896690225,
            10511150963279542933,
            241616963798111520,
            18006028873945532914,
            9331805924252967955,
            2302265547689836037,
            12963782374940901062,
            8219993065265687494,
            12618148012554300263,
            1039461056284508576,
        ],
        [
            12748361224490667899,
            10886323781082927488,
            13818727034541541046,
            8618987772202563484,
            14691541615571573852,
            4071646043153026265,
            18388952552347822950,
            16548167211803945576,
            4567596056723711950,
            8982355437907120602,
            3087135408424247086,
            995671111504812497,
            11325933967216566838,
            11612503906785408508,
            583487601176126367,
            10134816688150101140,
        ],
        [
            3375751975764758358,
            13056833532016612176,
            17756148044528524708,
            1889252512534425886,
            16398512735938907138,
            2959602061318052275,
            8966037174827344546,
            315821774502587490,
            10951989811044591525,
            946275430111047985,
            6991878258136628532,
            14439405831605097195,
            513397696341833602,
            6380612656745728045,
            12944181267444100810,
            1787549130703156564,
        ],
        [
            11614213899518253349,
            13246348992155029682,
            18336487506126531760,
            17624991145391587845,
            16458914885531477848,
            8029278087478881945,
            4180583130196688851,
            14731887832233212298,
            13532423566657311572,
            12714550641275206056,
            13955490643458269335,
            17750519083606805555,
            163823617960563388,
            1086330511530884246,
            16959909620709720150,
            17681070011044907595,
        ],
        [
            16599614249812962536,
            7803110853478675324,
            10318442219680384412,
            4153859382967112403,
            17912283481206044983,
            11781695110452132501,
            8090677405417809779,
            7815299112245249349,
            2251951358087237249,
            12194529946408573591,
            1085232231026213996,
            8437956482689942633,
            6855515007826624204,
            16819139425039084807,
            5604299142405863937,
            17796247882482643594,
        ],
        [
            5973572587936210280,
            454446004893024126,
            15136020193457492297,
            8332847059729611679,
            16383993793636841767,
            2283063635358088795,
            12274229289484807760,
            223116951882676051,
            12318788110324226082,
            12256134611310867298,
            15972169992173073850,
            3538272117638140233,
            9878993504305689627,
            3100512889429695061,
            7303867644064037781,
            5703926272569592334,
        ],
        [
            10961787868276860078,
            581305808160152665,
            16166788229076883096,
            1993660461189927881,
            8689758134718654344,
            14128632722310182820,
            6917176677716176055,
            7843315239059388895,
            2593778377522535279,
            18105072872355312268,
            8089281713009553299,
            14529271575185297307,
            15143216176571217277,
            16453070643203807917,
            2750375299700215259,
            9078883392150329377,
        ],
        [
            518683209083855798,
            8499862224794657428,
            5174348511977613985,
            13621239866341845175,
            4004782519400738321,
            17846551144144018044,
            6449815966966954550,
            9621617833198713520,
            5294966748606978954,
            13763315001798998437,
            15353999170371765508,
            1030737699480763043,
            7184166374747287334,
            11638249494509505520,
            12905980183480088553,
            12410674063797156907,
        ],
        [
            3828008321411687452,
            16408249498863596965,
            3762278266341401837,
            13485000029902179254,
            4853572507466702511,
            10695383423896516613,
            6410046548676736108,
            5732963654345586304,
            14684098812024171000,
            8116519990954694917,
            7640357254634876177,
            9645181962160604773,
            3802392580537877379,
            11932936492263810040,
            1896978594582113397,
            13576877308937005336,
        ],
        [
            15817049229878045952,
            12447363210167762750,
            9299453445103807899,
            319170194326038767,
            15661575453859851907,
            12964205996382193130,
            15288965810336549295,
            15842047255023074795,
            4660327677188670443,
            16087476289288431359,
            15533804703584174966,
            12841053247769156205,
            15794172459673405884,
            8663502249469278506,
            251694255113103543,
            8246913117092344751,
        ],
        [
            14597651205145607457,
            6052820560155952254,
            496190691548299770,
            16261273884140271418,
            804011653996009905,
            8440222693849144221,
            8831062895872052593,
            10662203457506937722,
            14343026755649846963,
            10984305786840700301,
            7323077524193347506,
            16878169108137248887,
            13647947764241642863,
            7372302833232188363,
            8088713631901178948,
            2765731582103196637,
        ],
    ]);
    let bb: Array1<u64> = arr1(&[
        10377891390615051891,
        2711160514027544751,
        937288573580517830,
        3140648552890902380,
        5723500278200484488,
        1353657883163948417,
        1758813917976085407,
        111679780748590656,
        12120210081435311146,
        4827286353513958651,
        8961295245787571723,
        5305623105212259808,
        15209629549799208454,
        15750773591601177458,
        11278494511036687173,
        15238565802317318561,
    ]);
    let p: u64 = 18446744073709551557;
    let x: Array1<u64> = arr1(&[
        2261857956393471552,
        970110712913482780,
        11682116003509215595,
        2922731371609781433,
        5075696228954815200,
        9288737156829919820,
        14070209033818141604,
        9589925945622079424,
        18430035085072024786,
        5313945181112939120,
        17373879092663555088,
        9636010487237727785,
        7368050193563707963,
        9278203451875275089,
        11877497075180052045,
        9899185772631201693,
    ]);
    b.iter(|| {
        let r = zp::solve(&a, &bb, p).unwrap();
        assert_eq!(r, x);
        r
    });
}
