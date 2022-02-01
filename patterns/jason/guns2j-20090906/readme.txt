This is an updated version of Dieter and Peter's Gun Collection for 
Conway's Game of Life, compiled by Jason Summers.

Nearly all of the guns from the original collection (pre-1999) were built 
by Dietrich Leithner and/or Peter Rott. Most of the more recent guns and 
improvements are by David Eppstein, Scot Ellison, Dave Greene, and Karel 
Suhajda. The Life technology used in the guns was developed by many 
different people.

This collection comes in two parts, which may be distributed separately:

guns1j.zip  -  Glider guns with periods up to 999.
guns2j.zip  -  Glider guns with periods 1000 - 9999.

The patterns in this collection are in an "RLE" format that is supported
by almost all current Life software.

The original collection also consisted of two parts, but was different in
that the guns were mostly in "ProLife" (*.plf) format, and the second
part included some extra copies of guns in the p100-999 range, and files
had the naming convention "kXXXX.plf" rather than "pXXXXX.plf". I have
removed the extra guns, renamed the files to follow the "pXXXXX" naming
convention, and converted all the guns to RLE (*.lif) format. The file
modification times have been preserved.

It should be noted that many of the higher period guns are somewhat
obsolete, and could easily be improved using modern Life technology.
The guns below period 1000 are, in general, more likely to be up to date.


The goal of this collection is to include the smallest glider gun for each 
period. Size is measured by the area of the bounding rectangle of the gun. 
Every cell that is a part of the rotor or stator of the gun must be 
included in the bounding rectangle, excluding only the output gliders. It 
should be noted that the population of the gun is irrelevant, and it is 
often easy to reduce the population at a cost of increasing the bounding 
rectangle.

This collection also serves as a way to showcase new Life technology. In 
particular, if you find something that can be used to make a smaller gun 
for some period than could otherwise be built, then you know you've found 
something new and useful.

Why measure by bounding box area, rather than some other measure like
cell population? Well, it wasn't my idea, but at least bounding box area
is unambiguous and easy to measure, and avoids certain loopholes that
other measures might have. For example, if you only considered cell
population, most of the high-period guns might end up being boring glider
relays that are extremely large in area but small in population.


In this collection, a glider gun must emit gliders in exactly one uniform 
stream. The period of the gun itself is irrelevant; only the effective 
period of the output gliders is considered.

If you manage to construct a smaller gun than one in this collection, 
please send me the pattern file, and I'll add it to the collection. 
(Please send the full pattern, as opposed to instructions on how to make 
one.) If you construct a gun for a period that is not currently in the 
collection, I'll include it if it is reasonably well-built, and optimized 
for bounding-box area.

If you send a large number of guns, it would be helpful if they are in the 
"canonical form" for this collection, which means:
 1) The gliders travel to the lower right.
 2) The pattern is in the last phase before a glider leaves the bounding
    box.
 3) The width is greater than or equal to the height.
 4) The initial pattern should recur exactly, except for the output
    gliders (and optional extra cells to mark the bounding box). Do not
    delete any extraneous sparks, etc.


Jason Summers <jason1@pobox.com>
http://entropymine.com/jason/life/
