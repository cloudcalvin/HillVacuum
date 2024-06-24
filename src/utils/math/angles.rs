//=======================================================================//
// IMPORTS
//
//=======================================================================//

use bevy::prelude::Vec2;

//=======================================================================//
// CONSTANTS
//
//=======================================================================//

#[allow(clippy::approx_constant)]
#[allow(clippy::unreadable_literal)]
const SIN_COS_TAN_LOOKUP: [(f32, f32, f32); 361] = [
    (0., 1., 0.),
    (0.017452406, 0.9998477, 0.017455066),
    (0.034899496, 0.99939084, 0.03492077),
    (0.05233596, 0.9986295, 0.052407783),
    (0.06975647, 0.9975641, 0.06992681),
    (0.08715574, 0.9961947, 0.08748866),
    (0.104528464, 0.9945219, 0.10510424),
    (0.12186935, 0.99254614, 0.12278456),
    (0.1391731, 0.99026805, 0.14054084),
    (0.15643448, 0.98768836, 0.15838444),
    (0.17364818, 0.9848077, 0.17632698),
    (0.190809, 0.98162717, 0.19438031),
    (0.2079117, 0.9781476, 0.21255657),
    (0.22495106, 0.97437006, 0.23086819),
    (0.2419219, 0.9702957, 0.249328),
    (0.25881904, 0.9659258, 0.2679492),
    (0.27563736, 0.9612617, 0.28674537),
    (0.2923717, 0.9563048, 0.30573067),
    (0.309017, 0.95105654, 0.3249197),
    (0.32556817, 0.94551855, 0.3443276),
    (0.34202012, 0.9396926, 0.36397022),
    (0.35836795, 0.9335804, 0.38386405),
    (0.3746066, 0.92718387, 0.40402624),
    (0.39073113, 0.92050487, 0.4244748),
    (0.40673664, 0.9135454, 0.4452287),
    (0.42261827, 0.90630776, 0.46630767),
    (0.43837115, 0.89879405, 0.4877326),
    (0.4539905, 0.8910065, 0.5095254),
    (0.46947157, 0.88294756, 0.53170943),
    (0.4848096, 0.8746197, 0.554309),
    (0.5, 0.8660254, 0.5773503),
    (0.5150381, 0.8571673, 0.6008606),
    (0.52991927, 0.8480481, 0.62486935),
    (0.54463905, 0.83867055, 0.6494076),
    (0.5591929, 0.8290376, 0.67450845),
    (0.57357645, 0.81915206, 0.70020753),
    (0.58778524, 0.809017, 0.7265426),
    (0.601815, 0.79863554, 0.75355405),
    (0.6156615, 0.7880108, 0.78128564),
    (0.6293204, 0.7771459, 0.80978405),
    (0.6427876, 0.76604444, 0.8390996),
    (0.656059, 0.7547096, 0.8692868),
    (0.6691306, 0.7431448, 0.9004041),
    (0.6819984, 0.7313537, 0.932515),
    (0.6946584, 0.7193398, 0.96568877),
    (0.70710677, 0.70710677, 1.),
    (0.7193398, 0.6946584, 1.0355303),
    (0.7313537, 0.6819984, 1.0723687),
    (0.74314487, 0.66913056, 1.1106126),
    (0.75470954, 0.656059, 1.1503683),
    (0.76604444, 0.64278764, 1.1917536),
    (0.7771459, 0.62932044, 1.2348971),
    (0.7880107, 0.6156615, 1.2799416),
    (0.79863554, 0.60181504, 1.3270448),
    (0.809017, 0.5877853, 1.3763818),
    (0.81915206, 0.57357645, 1.4281479),
    (0.82903755, 0.5591929, 1.482561),
    (0.83867055, 0.54463905, 1.5398648),
    (0.8480481, 0.52991927, 1.6003345),
    (0.8571673, 0.5150381, 1.6642796),
    (0.86602545, 0.49999997, 1.7320509),
    (0.8746197, 0.48480958, 1.804048),
    (0.88294756, 0.4694716, 1.8807262),
    (0.8910065, 0.45399052, 1.9626104),
    (0.89879405, 0.43837115, 2.0503037),
    (0.9063078, 0.42261824, 2.144507),
    (0.9135455, 0.4067366, 2.246037),
    (0.9205048, 0.3907312, 2.355852),
    (0.92718387, 0.37460664, 2.4750865),
    (0.9335804, 0.35836798, 2.605089),
    (0.9396926, 0.34202015, 2.7474773),
    (0.94551855, 0.32556814, 2.904211),
    (0.95105654, 0.30901697, 3.077684),
    (0.9563047, 0.29237178, 3.2708519),
    (0.9612617, 0.2756374, 3.487414),
    (0.9659258, 0.25881907, 3.7320504),
    (0.9702957, 0.2419219, 4.010781),
    (0.97437006, 0.22495104, 4.331476),
    (0.9781476, 0.20791166, 4.704631),
    (0.98162717, 0.19080906, 5.144552),
    (0.9848077, 0.17364822, 5.6712804),
    (0.98768836, 0.15643449, 6.3137507),
    (0.99026805, 0.1391731, 7.11537),
    (0.99254614, 0.121869326, 8.144348),
    (0.9945219, 0.10452842, 9.514368),
    (0.9961947, 0.087155804, 11.430045),
    (0.9975641, 0.06975651, 14.300658),
    (0.9986295, 0.052335974, 19.081131),
    (0.99939084, 0.034899496, 28.636255),
    (0.9998477, 0.017452383, 57.29004),
    (1., -0.00000004371139, -22877334.),
    (0.9998477, -0.017452352, -57.290142),
    (0.99939084, -0.034899462, -28.636282),
    (0.9986295, -0.05233594, -19.081142),
    (0.9975641, -0.06975648, -14.300665),
    (0.9961947, -0.08715577, -11.430049),
    (0.9945219, -0.10452851, -9.51436),
    (0.99254614, -0.12186929, -8.14435),
    (0.99026805, -0.13917308, -7.115371),
    (0.98768836, -0.15643445, -6.313752),
    (0.9848077, -0.1736482, -5.6712813),
    (0.98162717, -0.19080903, -5.144553),
    (0.9781476, -0.20791163, -4.704632),
    (0.97437006, -0.22495101, -4.3314767),
    (0.9702957, -0.24192187, -4.0107813),
    (0.9659258, -0.25881904, -3.732051),
    (0.9612617, -0.27563736, -3.4874144),
    (0.9563047, -0.29237175, -3.2708523),
    (0.95105654, -0.30901694, -3.0776842),
    (0.9455186, -0.3255681, -2.9042113),
    (0.9396926, -0.34202012, -2.7474775),
    (0.93358046, -0.35836795, -2.6050892),
    (0.92718387, -0.3746066, -2.4750867),
    (0.92050487, -0.39073116, -2.3558521),
    (0.9135455, -0.40673658, -2.2460372),
    (0.90630776, -0.42261833, -2.1445065),
    (0.89879405, -0.43837112, -2.050304),
    (0.8910066, -0.4539904, -1.9626111),
    (0.88294756, -0.46947157, -1.8807263),
    (0.8746197, -0.48480955, -1.8040481),
    (0.8660254, -0.50000006, -1.7320505),
    (0.8571673, -0.515038, -1.6642797),
    (0.84804803, -0.5299193, -1.6003342),
    (0.83867055, -0.54463905, -1.5398649),
    (0.8290376, -0.55919284, -1.4825612),
    (0.819152, -0.57357645, -1.4281479),
    (0.809017, -0.5877852, -1.3763821),
    (0.7986355, -0.6018151, -1.3270446),
    (0.7880108, -0.61566144, -1.2799417),
    (0.77714604, -0.6293203, -1.2348975),
    (0.76604444, -0.64278764, -1.1917535),
    (0.7547096, -0.65605897, -1.1503686),
    (0.7431448, -0.6691307, -1.1106124),
    (0.7313537, -0.6819983, -1.0723687),
    (0.7193399, -0.6946583, -1.0355306),
    (0.70710677, -0.70710677, -1.),
    (0.69465846, -0.7193397, -0.96568894),
    (0.6819983, -0.73135376, -0.932515),
    (0.6691306, -0.7431448, -0.90040416),
    (0.65605897, -0.75470966, -0.8692866),
    (0.64278764, -0.76604444, -0.83909965),
    (0.6293205, -0.77714586, -0.80978423),
    (0.61566144, -0.7880108, -0.7812856),
    (0.6018151, -0.7986355, -0.75355417),
    (0.5877852, -0.80901706, -0.7265424),
    (0.57357645, -0.81915206, -0.7002076),
    (0.559193, -0.8290375, -0.6745087),
    (0.54463905, -0.83867055, -0.64940757),
    (0.5299193, -0.84804803, -0.62486947),
    (0.515038, -0.8571673, -0.60086054),
    (0.50000006, -0.8660254, -0.5773503),
    (0.48480955, -0.8746198, -0.55430895),
    (0.46947157, -0.88294756, -0.53170943),
    (0.45399058, -0.89100647, -0.5095256),
    (0.43837112, -0.89879405, -0.48773253),
    (0.42261833, -0.90630776, -0.46630773),
    (0.40673658, -0.9135455, -0.4452286),
    (0.39073116, -0.92050487, -0.42447484),
    (0.3746067, -0.9271838, -0.40402636),
    (0.35836792, -0.93358046, -0.38386402),
    (0.3420202, -0.9396926, -0.36397034),
    (0.3255681, -0.9455186, -0.34432754),
    (0.30901703, -0.9510565, -0.32491973),
    (0.29237184, -0.9563047, -0.30573082),
    (0.27563736, -0.9612617, -0.2867454),
    (0.25881913, -0.9659258, -0.2679493),
    (0.24192186, -0.9702957, -0.24932796),
    (0.22495112, -0.97437006, -0.23086825),
    (0.20791161, -0.9781476, -0.21255648),
    (0.19080901, -0.98162717, -0.19438033),
    (0.1736483, -0.9848077, -0.1763271),
    (0.15643445, -0.98768836, -0.15838441),
    (0.13917318, -0.99026805, -0.14054091),
    (0.12186928, -0.99254614, -0.122784495),
    (0.104528494, -0.9945219, -0.10510427),
    (0.08715588, -0.99619466, -0.0874888),
    (0.06975647, -0.9975641, -0.069926806),
    (0.05233605, -0.9986295, -0.052407872),
    (0.03489945, -0.99939084, -0.034920722),
    (0.017452458, -0.9998477, -0.017455118),
    (-0.00000008742278, -1., 0.00000008742278),
    (-0.017452395, -0.9998477, 0.017455053),
    (-0.034899388, -0.99939084, 0.03492066),
    (-0.052335985, -0.9986295, 0.05240781),
    (-0.0697564, -0.9975641, 0.06992674),
    (-0.08715581, -0.9961947, 0.08748873),
    (-0.104528435, -0.9945219, 0.10510421),
    (-0.121869214, -0.99254614, 0.12278443),
    (-0.13917312, -0.99026805, 0.14054085),
    (-0.15643437, -0.98768836, 0.15838435),
    (-0.17364822, -0.9848077, 0.17632703),
    (-0.19080895, -0.98162717, 0.19438025),
    (-0.20791179, -0.97814757, 0.21255666),
    (-0.22495104, -0.97437006, 0.23086819),
    (-0.2419218, -0.9702957, 0.2493279),
    (-0.25881907, -0.9659258, 0.26794922),
    (-0.2756373, -0.9612617, 0.2867453),
    (-0.29237178, -0.9563047, 0.30573076),
    (-0.30901697, -0.95105654, 0.32491967),
    (-0.32556805, -0.9455186, 0.34432748),
    (-0.34202015, -0.9396926, 0.36397025),
    (-0.35836786, -0.93358046, 0.38386393),
    (-0.37460664, -0.9271838, 0.4040263),
    (-0.3907311, -0.92050487, 0.42447478),
    (-0.40673652, -0.9135455, 0.44522852),
    (-0.42261827, -0.9063078, 0.46630767),
    (-0.43837106, -0.8987941, 0.48773247),
    (-0.45399055, -0.8910065, 0.5095255),
    (-0.4694715, -0.8829476, 0.5317094),
    (-0.4848097, -0.87461966, 0.5543091),
    (-0.49999997, -0.8660254, 0.57735026),
    (-0.51503795, -0.85716736, 0.6008605),
    (-0.52991927, -0.8480481, 0.6248694),
    (-0.544639, -0.8386706, 0.6494075),
    (-0.55919296, -0.82903755, 0.6745086),
    (-0.5735764, -0.81915206, 0.7002075),
    (-0.5877851, -0.80901706, 0.7265423),
    (-0.60181504, -0.7986355, 0.75355405),
    (-0.6156614, -0.78801084, 0.78128546),
    (-0.62932044, -0.7771459, 0.8097841),
    (-0.6427876, -0.7660445, 0.8390995),
    (-0.6560591, -0.75470954, 0.8692869),
    (-0.6691306, -0.7431448, 0.90040404),
    (-0.68199825, -0.73135376, 0.9325149),
    (-0.6946584, -0.7193398, 0.9656888),
    (-0.7071067, -0.7071068, 0.9999999),
    (-0.71933985, -0.69465834, 1.0355304),
    (-0.7313537, -0.6819984, 1.0723686),
    (-0.74314475, -0.6691307, 1.1106123),
    (-0.7547096, -0.656059, 1.1503685),
    (-0.76604456, -0.6427875, 1.191754),
    (-0.777146, -0.6293203, 1.2348973),
    (-0.7880107, -0.6156615, 1.2799414),
    (-0.7986354, -0.60181516, 1.3270444),
    (-0.8090168, -0.5877854, 1.3763812),
    (-0.8191521, -0.57357633, 1.4281484),
    (-0.8290376, -0.5591929, 1.4825611),
    (-0.83867055, -0.5446391, 1.5398648),
    (-0.84804803, -0.5299194, 1.6003339),
    (-0.8571672, -0.5150383, 1.6642785),
    (-0.86602545, -0.4999999, 1.7320513),
    (-0.8746197, -0.4848096, 1.8040478),
    (-0.88294756, -0.46947163, 1.8807261),
    (-0.89100647, -0.45399067, 1.9626096),
    (-0.8987941, -0.43837097, 2.050305),
    (-0.9063078, -0.42261818, 2.1445074),
    (-0.9135454, -0.40673664, 2.2460368),
    (-0.9205048, -0.39073122, 2.3558517),
    (-0.9271838, -0.3746068, 2.4750855),
    (-0.9335805, -0.35836777, 2.6050904),
    (-0.9396927, -0.34202006, 2.747478),
    (-0.94551855, -0.32556817, 2.9042108),
    (-0.9510565, -0.3090171, 3.0776823),
    (-0.95630467, -0.2923719, 3.2708502),
    (-0.96126175, -0.2756372, 3.4874165),
    (-0.9659259, -0.25881898, 3.7320518),
    (-0.9702957, -0.24192193, 4.0107803),
    (-0.97437006, -0.22495118, 4.3314734),
    (-0.97814757, -0.20791192, 4.7046247),
    (-0.9816272, -0.19080885, 5.144558),
    (-0.9848078, -0.17364813, 5.6712832),
    (-0.9876883, -0.15643452, 6.3137493),
    (-0.99026805, -0.13917325, 7.115362),
    (-0.99254614, -0.121869594, 8.14433),
    (-0.9945219, -0.10452834, 9.514377),
    (-0.9961947, -0.087155715, 11.430057),
    (-0.997564, -0.069756545, 14.300652),
    (-0.9986295, -0.052336123, 19.081076),
    (-0.99939084, -0.034899764, 28.636034),
    (-0.9998477, -0.017452296, 57.290325),
    (-1., 0.000000011924881, -83858280.),
    (-0.9998477, 0.01745232, -57.29025),
    (-0.99939084, 0.03489931, -28.636406),
    (-0.9986295, 0.05233615, -19.081066),
    (-0.997564, 0.06975657, -14.300647),
    (-0.9961947, 0.08715574, -11.430053),
    (-0.9945219, 0.10452836, -9.514374),
    (-0.9925462, 0.12186914, -8.144361),
    (-0.99026805, 0.13917327, -7.1153607),
    (-0.9876883, 0.15643454, -6.3137484),
    (-0.9848077, 0.17364815, -5.671283),
    (-0.9816272, 0.19080888, -5.1445575),
    (-0.9781476, 0.20791148, -4.704635),
    (-0.97437, 0.22495121, -4.331473),
    (-0.9702957, 0.24192195, -4.01078),
    (-0.9659258, 0.258819, -3.7320514),
    (-0.96126175, 0.2756372, -3.4874163),
    (-0.95630485, 0.29237148, -3.2708554),
    (-0.9510565, 0.30901712, -3.077682),
    (-0.94551855, 0.3255682, -2.9042106),
    (-0.9396926, 0.3420201, -2.747478),
    (-0.93358046, 0.3583678, -2.6050904),
    (-0.9271839, 0.37460637, -2.4750886),
    (-0.9205048, 0.39073125, -2.3558517),
    (-0.9135454, 0.40673667, -2.2460365),
    (-0.9063078, 0.42261818, -2.1445074),
    (-0.8987941, 0.438371, -2.0503047),
    (-0.89100665, 0.45399025, -1.9626118),
    (-0.88294756, 0.46947166, -1.880726),
    (-0.8746197, 0.48480964, -1.8040477),
    (-0.86602545, 0.4999999, -1.7320511),
    (-0.8571674, 0.5150379, -1.6642802),
    (-0.848048, 0.52991945, -1.6003339),
    (-0.8386705, 0.5446391, -1.5398647),
    (-0.8290376, 0.5591929, -1.482561),
    (-0.8191521, 0.57357633, -1.4281484),
    (-0.8090171, 0.58778507, -1.3763826),
    (-0.7986354, 0.60181516, -1.3270444),
    (-0.7880107, 0.6156615, -1.2799414),
    (-0.777146, 0.6293204, -1.2348973),
    (-0.7660445, 0.6427875, -1.1917539),
    (-0.7547097, 0.65605885, -1.1503689),
    (-0.74314475, 0.66913074, -1.1106122),
    (-0.73135364, 0.6819984, -1.0723686),
    (-0.71933985, 0.69465834, -1.0355304),
    (-0.7071069, 0.70710665, -1.0000002),
    (-0.6946585, 0.7193396, -0.96568924),
    (-0.68199825, 0.7313538, -0.93251485),
    (-0.66913056, 0.74314487, -0.900404),
    (-0.6560591, 0.75470954, -0.86928684),
    (-0.64278775, 0.7660443, -0.8390999),
    (-0.6293206, 0.7771458, -0.8097845),
    (-0.6156614, 0.78801084, -0.78128546),
    (-0.601815, 0.79863554, -0.75355405),
    (-0.5877853, 0.80901694, -0.72654265),
    (-0.57357657, 0.81915194, -0.7002078),
    (-0.55919313, 0.8290374, -0.6745089),
    (-0.54463893, 0.8386706, -0.64940745),
    (-0.52991927, 0.8480481, -0.62486935),
    (-0.51503813, 0.85716724, -0.6008608),
    (-0.5000002, 0.8660253, -0.57735056),
    (-0.48480946, 0.8746198, -0.55430883),
    (-0.46947148, 0.8829476, -0.5317093),
    (-0.45399052, 0.8910065, -0.5095255),
    (-0.43837124, 0.898794, -0.48773274),
    (-0.42261845, 0.9063077, -0.4663079),
    (-0.4067365, 0.91354555, -0.4452285),
    (-0.39073107, 0.92050487, -0.42447475),
    (-0.37460664, 0.92718387, -0.40402627),
    (-0.35836807, 0.9335804, -0.3838642),
    (-0.34202036, 0.93969256, -0.3639705),
    (-0.32556802, 0.9455186, -0.34432745),
    (-0.30901694, 0.95105654, -0.32491964),
    (-0.29237175, 0.9563047, -0.30573073),
    (-0.2756375, 0.96126163, -0.28674555),
    (-0.25881928, 0.96592575, -0.26794946),
    (-0.24192177, 0.9702958, -0.24932787),
    (-0.22495103, 0.97437006, -0.23086816),
    (-0.20791176, 0.97814757, -0.21255663),
    (-0.19080916, 0.98162717, -0.19438048),
    (-0.17364845, 0.9848077, -0.17632726),
    (-0.15643436, 0.98768836, -0.15838432),
    (-0.13917309, 0.99026805, -0.14054082),
    (-0.12186943, 0.99254614, -0.12278465),
    (-0.10452865, 0.99452186, -0.105104424),
    (-0.08715603, 0.99619466, -0.08748895),
    (-0.06975638, 0.9975641, -0.06992672),
    (-0.052335963, 0.9986295, -0.052407786),
    (-0.0348996, 0.99939084, -0.034920875),
    (-0.017452609, 0.9998477, -0.017455269),
    (0., 1., 0.)
];

//=======================================================================//
// TRAITS
//
//=======================================================================//

pub(crate) trait FastSinCosTan
{
    #[must_use]
    fn fast_sin_cos(&self) -> (f32, f32);

    #[must_use]
    fn fast_tan(&self) -> f32;
}

impl FastSinCosTan for i8
{
    #[inline]
    fn fast_sin_cos(&self) -> (f32, f32)
    {
        let slot = i8_sin_cos_slot(*self);
        (slot.0, slot.1)
    }

    #[must_use]
    fn fast_tan(&self) -> f32 { i8_sin_cos_slot(*self).2 }
}

impl FastSinCosTan for i16
{
    #[inline]
    fn fast_sin_cos(&self) -> (f32, f32)
    {
        let (sin, cos, _) = i16_sin_cos_slot(*self);
        (*sin, *cos)
    }

    #[must_use]
    fn fast_tan(&self) -> f32 { i16_sin_cos_slot(*self).2 }
}

//=======================================================================//
// FUNCTIONS
//
//=======================================================================//

#[inline]
#[must_use]
fn i8_sin_cos_slot(angle: i8) -> &'static (f32, f32, f32) { i16_sin_cos_slot(i16::from(angle)) }

//=======================================================================//

#[inline]
#[must_use]
fn i16_sin_cos_slot(angle: i16) -> &'static (f32, f32, f32)
{
    let idx = if angle < 0 { 360 + angle } else { angle };
    &SIN_COS_TAN_LOOKUP[usize::try_from(idx).unwrap()]
}

//=======================================================================//

/// Computes the cosine of the angle of `v`.
#[inline]
#[must_use]
pub fn vector_angle_cosine(v: Vec2) -> f32 { v.normalize().dot(Vec2::X) }

//=======================================================================//

/// Computes the cosine of the angle between `vec_1` and `vec_2`.
#[inline]
#[must_use]
pub fn vectors_angle_cosine(vec_1: Vec2, vec_2: Vec2) -> f32
{
    vec_1.dot(vec_2) / (vec_1.length() * vec_2.length())
}
