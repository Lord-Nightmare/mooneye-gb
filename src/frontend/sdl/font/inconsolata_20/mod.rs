// This file is part of Mooneye GB.
// Copyright (C) 2014-2015 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// Mooneye GB is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Mooneye GB is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Mooneye GB.  If not, see <http://www.gnu.org/licenses/>.
use std::collections::BTreeMap;

pub const GLYPH_BYTES: &'static [u8] = include_bytes!("glyph.rgba");
pub const OUTLINE_BYTES: &'static [u8] = include_bytes!("outline.rgba");

pub fn offsets() -> BTreeMap<u32, (i32, i32)> {
  let mut map = BTreeMap::new();

  map.insert(32 , (0  , 0  ));
  map.insert(33 , (13 , 0  ));
  map.insert(34 , (26 , 0  ));
  map.insert(35 , (39 , 0  ));
  map.insert(36 , (52 , 0  ));
  map.insert(37 , (65 , 0  ));
  map.insert(38 , (78 , 0  ));
  map.insert(39 , (91 , 0  ));
  map.insert(40 , (104, 0  ));
  map.insert(41 , (117, 0  ));
  map.insert(42 , (130, 0  ));
  map.insert(43 , (143, 0  ));
  map.insert(44 , (156, 0  ));
  map.insert(45 , (169, 0  ));
  map.insert(46 , (182, 0  ));
  map.insert(47 , (195, 0  ));
  map.insert(48 , (208, 0  ));
  map.insert(49 , (221, 0  ));
  map.insert(50 , (234, 0  ));
  map.insert(51 , (0  , 23 ));
  map.insert(52 , (13 , 23 ));
  map.insert(53 , (26 , 23 ));
  map.insert(54 , (39 , 23 ));
  map.insert(55 , (52 , 23 ));
  map.insert(56 , (65 , 23 ));
  map.insert(57 , (78 , 23 ));
  map.insert(58 , (91 , 23 ));
  map.insert(59 , (104, 23 ));
  map.insert(60 , (117, 23 ));
  map.insert(61 , (130, 23 ));
  map.insert(62 , (143, 23 ));
  map.insert(63 , (156, 23 ));
  map.insert(64 , (169, 23 ));
  map.insert(65 , (182, 23 ));
  map.insert(66 , (195, 23 ));
  map.insert(67 , (208, 23 ));
  map.insert(68 , (221, 23 ));
  map.insert(69 , (234, 23 ));
  map.insert(70 , (0  , 46 ));
  map.insert(71 , (13 , 46 ));
  map.insert(72 , (26 , 46 ));
  map.insert(73 , (39 , 46 ));
  map.insert(74 , (52 , 46 ));
  map.insert(75 , (65 , 46 ));
  map.insert(76 , (78 , 46 ));
  map.insert(77 , (91 , 46 ));
  map.insert(78 , (104, 46 ));
  map.insert(79 , (117, 46 ));
  map.insert(80 , (130, 46 ));
  map.insert(81 , (143, 46 ));
  map.insert(82 , (156, 46 ));
  map.insert(83 , (169, 46 ));
  map.insert(84 , (182, 46 ));
  map.insert(85 , (195, 46 ));
  map.insert(86 , (208, 46 ));
  map.insert(87 , (221, 46 ));
  map.insert(88 , (234, 46 ));
  map.insert(89 , (0  , 69 ));
  map.insert(90 , (13 , 69 ));
  map.insert(91 , (26 , 69 ));
  map.insert(92 , (39 , 69 ));
  map.insert(93 , (52 , 69 ));
  map.insert(94 , (65 , 69 ));
  map.insert(95 , (78 , 69 ));
  map.insert(96 , (91 , 69 ));
  map.insert(97 , (104, 69 ));
  map.insert(98 , (117, 69 ));
  map.insert(99 , (130, 69 ));
  map.insert(100, (143, 69 ));
  map.insert(101, (156, 69 ));
  map.insert(102, (169, 69 ));
  map.insert(103, (182, 69 ));
  map.insert(104, (195, 69 ));
  map.insert(105, (208, 69 ));
  map.insert(106, (221, 69 ));
  map.insert(107, (234, 69 ));
  map.insert(108, (0  , 92 ));
  map.insert(109, (13 , 92 ));
  map.insert(110, (26 , 92 ));
  map.insert(111, (39 , 92 ));
  map.insert(112, (52 , 92 ));
  map.insert(113, (65 , 92 ));
  map.insert(114, (78 , 92 ));
  map.insert(115, (91 , 92 ));
  map.insert(116, (104, 92 ));
  map.insert(117, (117, 92 ));
  map.insert(118, (130, 92 ));
  map.insert(119, (143, 92 ));
  map.insert(120, (156, 92 ));
  map.insert(121, (169, 92 ));
  map.insert(122, (182, 92 ));
  map.insert(123, (195, 92 ));
  map.insert(124, (208, 92 ));
  map.insert(125, (221, 92 ));
  map.insert(126, (234, 92 ));
  map.insert(160, (0  , 115));
  map.insert(161, (13 , 115));
  map.insert(162, (26 , 115));
  map.insert(163, (39 , 115));
  map.insert(164, (52 , 115));
  map.insert(165, (65 , 115));
  map.insert(166, (78 , 115));
  map.insert(167, (91 , 115));
  map.insert(168, (104, 115));
  map.insert(169, (117, 115));
  map.insert(170, (130, 115));
  map.insert(171, (143, 115));
  map.insert(172, (156, 115));
  map.insert(173, (169, 115));
  map.insert(174, (182, 115));
  map.insert(175, (195, 115));
  map.insert(176, (208, 115));
  map.insert(177, (221, 115));
  map.insert(178, (234, 115));
  map.insert(179, (0  , 138));
  map.insert(180, (13 , 138));
  map.insert(181, (26 , 138));
  map.insert(182, (39 , 138));
  map.insert(183, (52 , 138));
  map.insert(184, (65 , 138));
  map.insert(185, (78 , 138));
  map.insert(186, (91 , 138));
  map.insert(187, (104, 138));
  map.insert(188, (117, 138));
  map.insert(189, (130, 138));
  map.insert(190, (143, 138));
  map.insert(191, (156, 138));
  map.insert(192, (169, 138));
  map.insert(193, (182, 138));
  map.insert(194, (195, 138));
  map.insert(195, (208, 138));
  map.insert(196, (221, 138));
  map.insert(197, (234, 138));
  map.insert(198, (0  , 161));
  map.insert(199, (13 , 161));
  map.insert(200, (26 , 161));
  map.insert(201, (39 , 161));
  map.insert(202, (52 , 161));
  map.insert(203, (65 , 161));
  map.insert(204, (78 , 161));
  map.insert(205, (91 , 161));
  map.insert(206, (104, 161));
  map.insert(207, (117, 161));
  map.insert(208, (130, 161));
  map.insert(209, (143, 161));
  map.insert(210, (156, 161));
  map.insert(211, (169, 161));
  map.insert(212, (182, 161));
  map.insert(213, (195, 161));
  map.insert(214, (208, 161));
  map.insert(215, (221, 161));
  map.insert(216, (234, 161));
  map.insert(217, (0  , 184));
  map.insert(218, (13 , 184));
  map.insert(219, (26 , 184));
  map.insert(220, (39 , 184));
  map.insert(221, (52 , 184));
  map.insert(222, (65 , 184));
  map.insert(223, (78 , 184));
  map.insert(224, (91 , 184));
  map.insert(225, (104, 184));
  map.insert(226, (117, 184));
  map.insert(227, (130, 184));
  map.insert(228, (143, 184));
  map.insert(229, (156, 184));
  map.insert(230, (169, 184));
  map.insert(231, (182, 184));
  map.insert(232, (195, 184));
  map.insert(233, (208, 184));
  map.insert(234, (221, 184));
  map.insert(235, (234, 184));
  map.insert(236, (0  , 207));
  map.insert(237, (13 , 207));
  map.insert(238, (26 , 207));
  map.insert(239, (39 , 207));
  map.insert(240, (52 , 207));
  map.insert(241, (65 , 207));
  map.insert(242, (78 , 207));
  map.insert(243, (91 , 207));
  map.insert(244, (104, 207));
  map.insert(245, (117, 207));
  map.insert(246, (130, 207));
  map.insert(247, (143, 207));
  map.insert(248, (156, 207));
  map.insert(249, (169, 207));
  map.insert(250, (182, 207));
  map.insert(251, (195, 207));
  map.insert(252, (208, 207));
  map.insert(253, (221, 207));
  map.insert(254, (234, 207));
  map.insert(255, (0  , 230));

  map
}

