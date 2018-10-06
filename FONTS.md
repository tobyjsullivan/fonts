# Fonts

## History

### PostScript Type 1
Adobe released the PostScript language in 1984 as a tool for translating digital documents into print.[3]
PostScript is a high-level language which describes documents as scripts.[1]
These scripts are read by device-specific PostScript interpretters which produce device-appropriate raster data.

PostScript Type 1 "font programs" were created by Adobe to represent letter and symbol shapes.[2]
This font format was effectively a modified version of PostScript and also revolved around the execution of descriptive scripts.

### Compact Font Format (CFF)
The CFF format was introduced by Adobe to allow a single file to include multiple PostScript fonts in a "FontSet".[5]

### TrueType
Apple released the TrueType font format in 1991.[0]

### SFNT
The original table-based structure of the TrueType font format was eventually exapted to support other file formats such as PostScript Type 1 and platforms such as Windows. This extended use led to the adoption of a more general term for the format, SFNT (spline font).[0]

### OpenType
OpenType is an extension of the TrueType format.[4]

### WOFF
The World Wide Web Consortium (W3C) introduced WOFF 1.0 to make font files a little bit more friendly for web usage.[6]
A WOFF format font is just a regular OpenType font wrapped in an envelope and compressed.
For this reason, any OpenType font can be converted to a WOFF with relative ease (wrap and compress).

### WOFF2
After the success of WOFF 1.0, the W3C introduced WOFF 2.0 with improved compression.[7]

## SFNT (Spline Font)
Since the introduction of TrueType in 1991, most fonts have used the same table-based file structure known as `sfnt` in some way.
The `sfnt` file format consists of a binary header section followed by a series of concatenated tables (described in the header).
TrueType, OpenType, and a PostScript-compatible font introduced by Apple (name yet to be discovered) all leverage this format exactly with the only meaningful variance being the specific information tables.
The modern WOFF fonts (both 1.0 and 2.0) wrap any `sfnt` font in an envelope and apply compression.

It's worth noting that the format inception doesn't stop there.
The primary reason `sfnt` became so popular in the first place, and continues to be popular, is the unbounded flexibility of the tables - any data can be stored in a table.
In fact, the PostScript-compatible `sfnt` font introduced by Apple simply took existing PostScript Type 1 fonts and stuffed them, verbatim, into a table tagged `post` so that it could be available for PostScript printers when needed.[9]
Likewise, a later font format from Adobe called CFF is frequently put in a table called `CFF`.
Eventually, the OpenType spec went so far as to define a `CFF2` table which contains the subset of the aforementioned `CFF` data which does not overlap with other, existing OpenType tables.[8]

## Drawing (Spline Fonts)
> See Apple's documentation on [Digitizing Letter Forms](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM01/Chap1.html) for a full and thorough explanation of the concepts in this section.
> That explanation is more complete than what you will find here.

This section will explore how TrueType fonts (including OpenType fonts with TrueType outlines) are represented and ultiamtely drawn to the screen.

Every font is effectively a set of visual glyphs which are mapped to written language characters and symbols.
As a basic example, the font you are reading has a visual representation of the letter "A" which is somehow interpretted and drawn to your screen.
In the case of the latin letter "A", there is a single glyph.
In other cases, such as the accented letter "Á", there might be two glyphs (the base character, "A", and the accent) which are combined into a "compound glyph" for rendering.

The more common mechanism for representing fonts is TrueType outlines.
These outlines are effectively a series of "splines" (e.g., connected curves) which make up the outline of a glyph.
These splines are made up of Bezier curves which, together, outline given character in a closed contour.

Once you have a contour for a glyph, you can determine which areas are inside the contour and shade those regions.
At this point, you have a drawn character.

As you may have realised, there are quite a few more technical details than the above but that does effectively capture the jist of drawing characters.

### Unit Conversions
Font glyphs are presented on a plane of arbitrary size (decided by the font manufacturer) known as the "master grid".
This grid is made up of 32767x32767 indivisible units called font-units.
Each point on the grid is referenced by a cartesian coordinate system which spans from -16384 to +16383 in both the horizontal and vertical directions (x-axis and y-axis).

The key area of this plane is known as the "em-square" (a metaphore of the old typographic concept).
Unlike in physical typesetting, modern glyphs are not strictly bound to the em-square.
The font manufacturer defines the size of the em-square in font units.

To display a glyph on a screen or in print, it must be scaled to the appropriate size.
This is done by finding the appropriate scale of the master grid to the new medium.

The following variables factor into our scale calculation:
- `point_size`: The actual font size we want to display (think "12-point font").
   This is context-dependent such as when an application defines a font size for display.
- `resolution`: The display resolution of our output device in pixels per inch.
- `units_per_em`: The number of font-units per em-square as defined in the font.

The scaling factor can then be calculated as:

> `pointSize * resolution / (72 points per inch * units_per_em)`[10]

The `72` here is a constant which represents the base font size.
This is standard for any TrueType glyph.

## Glossary

**Font-Unit**

An indivisible point of an arbitrary size which makes up the master grid.

**Glyph**

An elemental symbol which represents a whole or a part of a readable character.

**Master Grid**

The finite-sized plane of font-unit points on which glyphs are defined.

**Outline**

See: **TrueType Outline**

**TrueType Outline**

The set of one or more contours which describe the visual shape of a glyph in a TrueType font.

## References

- [0] Apple. _The sfnt format_. Webpage. Accessed 2018-10-03. https://web.archive.org/web/20131023054643/https://developer.apple.com/fonts/tools/tooldir/TrueEdit/Documentation/TE/TE1sfnt.html.
- [1] Adobe. _PostScript Language Reference_. 2nd ed. PDF File. Accessed 2018-10-03. https://www.adobe.com/content/dam/acom/en/devnet/actionscript/articles/psrefman.pdf.
- [2] Adobe. _Adobe Type 1 Font Format_. PDF File. Accessed 2018-10-03. https://www-cdf.fnal.gov/offline/PostScript/T1_SPEC.PDF.
- [3] Adobe. _Adobe PostScript_. Webpage. Accessed 2018-10-03. https://www.adobe.com/products/postscript.html.
- [4] Microsoft. _TrueType Fundamentals_. Webpage. Accessed 2018-10-03. https://docs.microsoft.com/en-us/typography/opentype/spec/ttch01.
- [5] Adobe. _The Compact Font Format Specification_. PDF File. Accessed 2018-10-03. http://wwwimages.adobe.com/www.adobe.com/content/dam/acom/en/devnet/font/pdfs/5176.CFF.pdf.
- [6] W3C. _WOFF File Format 1.0_. Webpage. Accessed 2018-10-04. https://www.w3.org/TR/2012/REC-WOFF-20121213/.
- [7] W3C. _WOFF File Format 2.0_. Webpage. Accessed 2018-10-04. https://www.w3.org/TR/WOFF2/.
- [8] Microsoft. _CFF2 - Compact Font Format (CFF) Version 2_. Webpage. Accessed 2018-10-04. https://docs.microsoft.com/en-us/typography/opentype/spec/cff2.
- [9] Microsoft. _post - PostScript Table_. Webpage. Accessed 2018-10-04. https://docs.microsoft.com/en-us/typography/opentype/spec/post.
- [10] Apple. _The Font Engine_. Webpage. Accessed 2018-10-06. https://developer.apple.com/fonts/TrueType-Reference-Manual/RM02/Chap2.html.

## Resources

- Adobe. 'Adobe PostScript'. https://www.adobe.com/products/postscript.html.
- Adobe. 'Adobe Type 1 Font Format'. https://www-cdf.fnal.gov/offline/PostScript/T1_SPEC.PDF.
- Adobe. 'PostScript Language Reference'. 2nd Ed. https://www.adobe.com/content/dam/acom/en/devnet/actionscript/articles/psrefman.pdf.
- Adobe. 'PostScript Language Reference'. 3rd Ed. https://www.adobe.com/content/dam/acom/en/devnet/actionscript/articles/PLRM.pdf.
- Adobe. 'The Compact Font Format Specification'. http://wwwimages.adobe.com/www.adobe.com/content/dam/acom/en/devnet/font/pdfs/5176.CFF.pdf.
- Apple. 'The sfnt format'. https://web.archive.org/web/20131023054643/https://developer.apple.com/fonts/tools/tooldir/TrueEdit/Documentation/TE/TE1sfnt.html.
- Apple. 'TrueType Reference Manual'. https://developer.apple.com/fonts/TrueType-Reference-Manual/.
- Microsoft. 'OpenType® specification'. https://docs.microsoft.com/en-us/typography/opentype/spec/.
- Microsoft. 'TrueType Fundamentals'. https://docs.microsoft.com/en-us/typography/opentype/spec/ttch01.
- W3C. 'WOFF File Format 1.0'. https://www.w3.org/TR/2012/REC-WOFF-20121213/.
- W3C. 'WOFF File Format 2.0'. https://www.w3.org/TR/WOFF2/.
