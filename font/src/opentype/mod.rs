mod tables;
mod types;

use std::fmt::Debug;

use self::tables::cmap::CmapTable;
use self::tables::head::HeadTable;
use self::tables::loca::LocaTable;
use self::tables::maxp::MaxpTable;
use self::tables::name::NameTable;
use super::sfnt::SfntFile;

#[derive(Debug)]
pub struct OpenTypeFile<'a> {
    sfnt: SfntFile<'a>,
    cmap: Option<CmapTable>,
    head: Option<HeadTable>,
    loca: Option<LocaTable>,
    maxp: Option<MaxpTable>,
    name: Option<NameTable<'a>>,
}

impl<'a> OpenTypeFile<'a> {
    pub fn deserialize(content: &'a [u8]) -> Self {
        let sfnt = SfntFile::deserialize(content);

        let mut cmap_data = None;
        let mut head_data = None;
        let mut loca_data = None;
        let mut maxp_data = None;
        let mut name_data = None;

        for record in &sfnt.tables {
          let table_type = TableType::table_type(record.tag);
          match table_type {
            TableType::Cmap => { cmap_data = Some(record.table_data); }
            TableType::Head => { head_data = Some(record.table_data); }
            TableType::Loca => { loca_data = Some(record.table_data); }
            TableType::Maxp => { maxp_data = Some(record.table_data); }
            TableType::Name => { name_data = Some(record.table_data); }
            _ => {}
          }
        }

        let mut cmap = None;
        if let Some(table_data) = cmap_data {
          match CmapTable::deserialize(table_data) {
            Ok(parsed) => { cmap = Some(parsed); }
            Err(err) => { panic!("Error deserializing cmap table {:?}", err); }
          }
        }
        let mut head = None;
        if let Some(table_data) = head_data {
          head  = Some(HeadTable::parse(table_data));
        }
        let mut maxp = None;
        if let Some(table_data) = maxp_data {
          maxp  = Some(MaxpTable::parse(table_data));
        }
        let mut loca = None;
        if let Some(table_data) = loca_data {
          maxp = match maxp {
            Some(maxp_table) => {
              loca  = Some(LocaTable::parse(table_data, maxp_table.version, maxp_table.num_glyphs));
              Some(maxp_table)
            }
            None => { panic!("Cannot deserialize loca table because no maxp table found."); }
          }
          
        }
        let mut name = None;
        if let Some(table_data) = name_data {
          match NameTable::deserialize(table_data) {
            Ok(parsed) => { name = Some(parsed); }
            Err(err) => { panic!("Error deserializing cmap table {:?}", err); }
          }
        }

        Self {
            sfnt,
            cmap,
            head,
            loca,
            maxp,
            name,
        }
    }

    fn deserialize_table<T: Debug, E: Debug>(
        data: &'a [u8],
        deserialize_fn: &Fn(&'a [u8]) -> Result<T, E>,
    ) -> T {
        match deserialize_fn(data) {
            Ok(table) => table,
            Err(err) => panic!("Error deserializing name table {:?}", err),
        }
    }
}

#[derive(PartialEq, Debug)]
enum TableType {
    /// Axis variations table
    Avar,
    /// Baseline table
    Base,
    /// Color bitmap data table
    Cbdt,
    /// Color bitmap location table
    Cblc,
    /// Compact font format (CFF) table
    Cff,
    /// Compact font format (CFF) version 2 table
    Cff2,
    /// Character to glyph index mapping table
    Cmap,
    /// Color table
    Colr,
    /// Color palette table
    Cpal,
    /// CVT variations table
    Cvar,
    /// Control value table
    Cvt,
    /// Digital signature table
    Dsig,
    /// Embedded bitmap data table
    Ebdt,
    /// Embedded bitmap location table
    Eblc,
    /// Embedded bitmap scaling table
    Ebsc,
    /// Font program table
    Fpgm,
    /// Font variations table
    Fvar,
    /// Grid-fitted and scan-conversion procedure table
    Gasp,
    /// Glyph definition table
    Gdef,
    /// Glyph data table
    Glyf,
    /// Glyph position table
    Gpos,
    /// Glyph substitution table
    Gsub,
    /// Glyph variations table
    Gvar,
    /// Horizontal device metrics table
    Hdmx,
    /// Font header table
    Head,
    /// Horizontal header table
    Hhea,
    /// Horizontal metrics table
    Hmtx,
    /// Horizontal metrics variations table
    Hvar,
    /// Justification table
    Jstf,
    /// Kerning table
    Kern,
    /// Index to location table
    Loca,
    /// Linear threshold table
    Ltsh,
    /// Mathmatical typesetting table
    Math,
    /// Maximum profile table
    Maxp,
    /// Merge table
    Merg,
    /// Metadata table
    Meta,
    /// Metrics variations table
    Mvar,
    /// Naming table
    Name,
    /// OS/2 and Windows metrics table
    Os2,
    /// PCL 5 table
    Pclt,
    /// PostScript table
    Post,
    /// Control value program table
    Prep,
    /// Standard bitmap graphics table
    Sbix,
    /// Style attributes table
    Stat,
    /// SVG table
    Svg,
    /// Vertical device metrics table
    Vdmx,
    /// Vertical header table
    Vhea,
    /// Vertical metrics table
    Vmtx,
    /// Vertical origin table
    Vorg,
    /// Vertical metrics variations table
    Vvar,
    /// A placeholder for any unrecognised table.
    Unknown,
}

impl TableType {
    fn table_type(tag: [char; 4]) -> Self {
        match tag {
            // Tags with fewer than four characters, such as cvt, pad spaces on the end.
            ['a', 'v', 'a', 'r'] => TableType::Avar,
            ['B', 'A', 'S', 'E'] => TableType::Base,
            ['C', 'B', 'D', 'T'] => TableType::Cbdt,
            ['C', 'B', 'L', 'C'] => TableType::Cblc,
            ['C', 'F', 'F', ' '] => TableType::Cff,
            ['C', 'F', 'F', '2'] => TableType::Cff2,
            ['c', 'm', 'a', 'p'] => TableType::Cmap,
            ['C', 'O', 'L', 'R'] => TableType::Colr,
            ['C', 'P', 'A', 'L'] => TableType::Cpal,
            ['c', 'v', 'a', 'r'] => TableType::Cvar,
            ['c', 'v', 't', ' '] => TableType::Cvt,
            ['D', 'S', 'I', 'G'] => TableType::Dsig,
            ['E', 'B', 'D', 'T'] => TableType::Ebdt,
            ['E', 'B', 'L', 'C'] => TableType::Eblc,
            ['E', 'B', 'S', 'C'] => TableType::Ebsc,
            ['f', 'p', 'g', 'm'] => TableType::Fpgm,
            ['f', 'v', 'a', 'r'] => TableType::Fvar,
            ['g', 'a', 's', 'p'] => TableType::Gasp,
            ['G', 'D', 'E', 'F'] => TableType::Gdef,
            ['g', 'l', 'y', 'f'] => TableType::Glyf,
            ['G', 'P', 'O', 'S'] => TableType::Gpos,
            ['G', 'S', 'U', 'B'] => TableType::Gsub,
            ['g', 'v', 'a', 'r'] => TableType::Gvar,
            ['h', 'd', 'm', 'x'] => TableType::Hdmx,
            ['h', 'e', 'a', 'd'] => TableType::Head,
            ['h', 'h', 'e', 'a'] => TableType::Hhea,
            ['h', 'm', 't', 'x'] => TableType::Hmtx,
            ['H', 'V', 'A', 'R'] => TableType::Hvar,
            ['J', 'S', 'T', 'F'] => TableType::Jstf,
            ['k', 'e', 'r', 'n'] => TableType::Kern,
            ['l', 'o', 'c', 'a'] => TableType::Loca,
            ['L', 'T', 'S', 'H'] => TableType::Ltsh,
            ['M', 'A', 'T', 'H'] => TableType::Math,
            ['m', 'a', 'x', 'p'] => TableType::Maxp,
            ['M', 'E', 'R', 'G'] => TableType::Merg,
            ['m', 'e', 't', 'a'] => TableType::Meta,
            ['M', 'V', 'A', 'R'] => TableType::Mvar,
            ['n', 'a', 'm', 'e'] => TableType::Name,
            ['O', 'S', '/', '2'] => TableType::Os2,
            ['p', 'c', 'l', 't'] => TableType::Pclt,
            ['p', 'o', 's', 't'] => TableType::Post,
            ['p', 'r', 'e', 'p'] => TableType::Prep,
            ['s', 'b', 'i', 'x'] => TableType::Sbix,
            ['S', 'T', 'A', 'T'] => TableType::Stat,
            ['S', 'V', 'G', ' '] => TableType::Svg,
            ['V', 'D', 'M', 'X'] => TableType::Vdmx,
            ['v', 'h', 'e', 'a'] => TableType::Vhea,
            ['v', 'm', 't', 'x'] => TableType::Vmtx,
            ['V', 'O', 'R', 'G'] => TableType::Vorg,
            ['V', 'V', 'A', 'R'] => TableType::Vvar,
            _ => TableType::Unknown,
        }
    }
}
