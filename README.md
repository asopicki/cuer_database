# README

This is a small script to quickly extract metadata from cuecards written 
in commonmark format. See http://commonmark.org for further details 
on the file format. Read below for the requirements on the file format
to be properly indexed in the database.

## Format

The cuecards have to be written in a special format in order for the
indexer to recognize the required data. First every cuecard needs
a title which is just the level 1 title in commonmark and *must* be on
the first line of the file.

### Example:

> `# My title`



## Development

Plan:
1. Setup pipeline to process all files 
1. Define relevant metadata **done**
  - Title
  - Choreographer
  - Rhythm
  - Phase
  - Steplevel
  - Difficulty
  - Music
1. Write filters to extract metadata 
1. Use metadata in output 
1. Write output to a json file
1. Define database schema
1. Write output to database 