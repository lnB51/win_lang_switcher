pub fn locale_from_u16(value: String) -> String {
    match value.as_str() {
        "00140C00" => "AD".to_owned(),
        "0000041C" => "SQ".to_owned(),
        "00000401" => "AR".to_owned(),
        "00010401" => "AR101".to_owned(),
        "00020401" => "AR102".to_owned(),
        "0000042B" => "HY".to_owned(),
        "0002042B" => "HY2".to_owned(),
        "0003042B" => "HY3".to_owned(),
        "0001042B" => "HY4".to_owned(),
        "0000044D" => "AS".to_owned(),
        "0001042C" => "AZ".to_owned(),
        "0000082C" => "AZCYR".to_owned(),
        "0000042C" => "AZLAT".to_owned(),
        "00000445" => "BN".to_owned(),
        "00020445" => "BNINS".to_owned(),
        "00010445" => "BNINSLEG".to_owned(),
        "0000046D" => "BA".to_owned(),
        "00000423" => "BE".to_owned(),
        "0001080C" => "BECO".to_owned(),
        "00000813" => "BEPE".to_owned(),
        "0000080C" => "BEFR".to_owned(),
        "0000201A" => "BSCYR".to_owned(),
        "000B0C00" => "BUG".to_owned(),
        "00030402" => "BG".to_owned(),
        "00010402" => "BGLAT".to_owned(),
        "00040402" => "BGPHT".to_owned(),
        "00020402" => "BGPH".to_owned(),
        "00000402" => "BGTY".to_owned(),
        "00001009" => "CAFR".to_owned(),
        "00000C0C" => "CAFRLEG".to_owned(),
        "00011009" => "CAMUL".to_owned(),
        "0000085F" => "CATA".to_owned(),
        "00000492" => "CKB".to_owned(),
        "0000045C" => "CHR".to_owned(),
        "0001045C" => "CHRPHON".to_owned(),
        "00000804" => "ZH".to_owned(),
        "00001004" => "ZHSG".to_owned(),
        "00000404" => "ZHTW".to_owned(),
        "00000C04" => "ZHTWHK".to_owned(),
        "00001404" => "ZHTWMO".to_owned(),
        "00000405" => "CS".to_owned(),
        "00010405" => "CSQW".to_owned(),
        "00020405" => "CSPROG".to_owned(),
        "00000406" => "DA".to_owned(),
        "00000439" => "HI".to_owned(),
        "00000465" => "DVPH".to_owned(),
        "00010465" => "DVTP".to_owned(),
        "00000413" => "NL".to_owned(),
        "00000C51" => "DZ".to_owned(),
        "00004009" => "INEN".to_owned(),
        "00000425" => "ET".to_owned(),
        "00000438" => "FO".to_owned(),
        "0000040B" => "FI".to_owned(),
        "0001083B" => "FISAM".to_owned(),
        "0000040C" => "FR".to_owned(),
        "00120C00" => "FU".to_owned(),
        "00020437" => "GEER".to_owned(),
        "00000437" => "GEL".to_owned(),
        "00030437" => "GEMES".to_owned(),
        "00040437" => "GEOL".to_owned(),
        "00010437" => "GEQW".to_owned(),
        "00000407" => "DE".to_owned(),
        "00010407" => "DEIBM".to_owned(),
        "000C0C00" => "GO".to_owned(),
        "00000408" => "GR".to_owned(),
        "00010408" => "GR220".to_owned(),
        "00030408" => "GR220LAT".to_owned(),
        "00020408" => "GR319".to_owned(),
        "00040408" => "GR319LAT".to_owned(),
        "00050408" => "GRLAT".to_owned(),
        "00060408" => "GRPOLY".to_owned(),
        "0000046F" => "GL".to_owned(),
        "00000474" => "GN".to_owned(),
        "00000447" => "GU".to_owned(),
        "00000468" => "HA".to_owned(),
        "00000475" => "HW".to_owned(),
        "0000040D" => "HE".to_owned(),
        "0002040D" => "HEST".to_owned(),
        "00010439" => "HITR".to_owned(),
        "0000040E" => "HU".to_owned(),
        "0001040E" => "HU101".to_owned(),
        "0000040F" => "IS".to_owned(),
        "00000470" => "IG".to_owned(),
        "0000085D" => "IKL".to_owned(),
        "0001045D" => "IKN".to_owned(),
        "00001809" => "GA".to_owned(),
        "00000410" => "IT".to_owned(),
        "00010410" => "IT142".to_owned(),
        "00000411" => "JA".to_owned(),
        "00110C00" => "JV".to_owned(),
        "0000044B" => "KA".to_owned(),
        "0000043F" => "KK".to_owned(),
        "00000453" => "KM".to_owned(),
        "00010453" => "KMNIDA".to_owned(),
        "00000412" => "KO".to_owned(),
        "00000440" => "KY".to_owned(),
        "00000454" => "LO".to_owned(),
        "0000080A" => "LATAM".to_owned(),
        "00000426" => "LV".to_owned(),
        "00010426" => "LVQW".to_owned(),
        "00020426" => "LVSTD".to_owned(),
        "00070C00" => "LSB".to_owned(),
        "00080C00" => "LSBSTD".to_owned(),
        "00010427" => "LT".to_owned(),
        "00000427" => "LTIBM".to_owned(),
        "00020427" => "LTSTD".to_owned(),
        "0000046E" => "LU".to_owned(),
        "0000042F" => "MK".to_owned(),
        "0001042F" => "MKSTD".to_owned(),
        "0000044C" => "ML".to_owned(),
        "0000043A" => "MT47".to_owned(),
        "0001043A" => "MT48".to_owned(),
        "00000481" => "MI".to_owned(),
        "0000044E" => "MR".to_owned(),
        "00000850" => "MNMS".to_owned(),
        "00000450" => "MNCYR".to_owned(),
        "00010C00" => "MM".to_owned(),
        "00130C00" => "MMV".to_owned(),
        "00001409" => "NZ".to_owned(),
        "00000461" => "NE".to_owned(),
        "00020C00" => "NTL".to_owned(),
        "00000414" => "NO".to_owned(),
        "0000043B" => "NOSAM".to_owned(),
        "00090C00" => "NK".to_owned(),
        "00000448" => "OD".to_owned(),
        "00040C00" => "OG".to_owned(),
        "000D0C00" => "OC".to_owned(),
        "000F0C00" => "OI".to_owned(),
        "00150C00" => "OS".to_owned(),
        "000E0C00" => "OSM".to_owned(),
        "00000463" => "PS".to_owned(),
        "00000429" => "FA".to_owned(),
        "00050429" => "FASTD".to_owned(),
        "000A0C00" => "PH".to_owned(),
        "00010415" => "PL214".to_owned(),
        "00000415" => "PL".to_owned(),
        "00000816" => "PT".to_owned(),
        "00000416" => "PTBR".to_owned(),
        "00010416" => "PTBRABNT2".to_owned(),
        "00000446" => "PA".to_owned(),
        "00000418" => "RO".to_owned(),
        "00020418" => "ROPROG".to_owned(),
        "00010418" => "ROSTD".to_owned(),
        "00000419" => "💩".to_owned(),
        "00010419" => "RUTY".to_owned(),
        "00020419" => "RUMN".to_owned(),
        "00000485" => "SA".to_owned(),
        "0002083B" => "SAMI".to_owned(),
        "0001043B" => "SAMINOR".to_owned(),
        "00011809" => "SG".to_owned(),
        "00000C1A" => "SRSC".to_owned(),
        "0000081A" => "SRLAT".to_owned(),
        "0000046C" => "SS".to_owned(),
        "00000432" => "TS".to_owned(),
        "0000045B" => "SI".to_owned(),
        "0001045B" => "SIW9".to_owned(),
        "0000041B" => "SK".to_owned(),
        "0001041B" => "SKQW".to_owned(),
        "00000424" => "SL".to_owned(),
        "00100C00" => "SO".to_owned(),
        "0001042E" => "SOREXT".to_owned(),
        "0002042E" => "SOREXTSTD".to_owned(),
        "0000042E" => "SORSTD".to_owned(),
        "0000040A" => "ES".to_owned(),
        "0001040A" => "ESVAR".to_owned(),
        "0000041A" => "STND".to_owned(),
        "0000041D" => "SE".to_owned(),
        "0000083B" => "SAMI".to_owned(),
        "0000100C" => "SWFR".to_owned(),
        "00000807" => "SWGR".to_owned(),
        "0000045A" => "SY".to_owned(),
        "0001045A" => "SYPH".to_owned(),
        "00030C00" => "TL".to_owned(),
        "00000428" => "TJ".to_owned(),
        "00000449" => "TA".to_owned(),
        "00020449" => "TA99".to_owned(),
        "00030449" => "TAANJ".to_owned(),
        "00010444" => "TT".to_owned(),
        "00000444" => "TTLEG".to_owned(),
        "0000044A" => "TE".to_owned(),
        "0000041E" => "TH".to_owned(),
        "0002041E" => "THKL".to_owned(),
        "0001041E" => "THPC".to_owned(),
        "0003041E" => "THPCNSL".to_owned(),
        "00000451" => "TI".to_owned(),
        "00010451" => "TIUP".to_owned(),
        "0000105F" => "TFB".to_owned(),
        "0001105F" => "TFEX".to_owned(),
        "00010850" => "TMSTD".to_owned(),
        "0001041F" => "TRF".to_owned(),
        "0000041F" => "TRQ".to_owned(),
        "00000442" => "TK".to_owned(),
        "00000409" => "US".to_owned(),
        "00050409" => "USAR".to_owned(),
        "00000422" => "UA".to_owned(),
        "00020422" => "UA".to_owned(),
        "00000809" => "GB".to_owned(),
        "00000452" => "GBEX".to_owned(),
        "00010409" => "USDV".to_owned(),
        "00030409" => "USDVLH".to_owned(),
        "00040409" => "USDVRH".to_owned(),
        "00020409" => "USIN".to_owned(),
        "00000420" => "UR".to_owned(),
        "00010480" => "UG".to_owned(),
        "00000480" => "UGLEG".to_owned(),
        "00000843" => "UZC".to_owned(),
        "0000042A" => "VI".to_owned(),
        "00000488" => "WO".to_owned(),
        "0000046A" => "YO".to_owned(),
        _ => "".to_owned(),
    }
}
