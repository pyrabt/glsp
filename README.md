# glsp [![Build Status](https://travis-ci.com/pyrabt/glsp.svg?branch=master)](https://travis-ci.com/pyrabt/glsp)

## Description
A command line tool that utilizes [Language Server Protocol](https://microsoft.github.io/language-server-protocol/overview) to identify files/directories which match the given "regex" by the user. An example of a command would be "lsgrep -Java --class foo". **LSP** is a standardization of communication between a development tool and a language server using JSON-RPC messaging. Language servers have been developed for most popular languages and support a large amount of development tools because of the standardization of the protocol.

There are two potential approaches I am exploring for the use of the language server component. The first, is to spin up the language server everytime the user makes a call to the cli app. The second, would be to have the language server(s) potentially running in the background in a separate process. Although the first appears superior, I will need to do some testing to determine the resource costs for each approach.

On top of these two approaches, I will also need to test whether parsing and generating the JSON cache file for the server is better done each time the app is called in a new directory, or if i should generate this cache on a rolling basis from the specified root of the user's project filesystem tree. 

## Final Deliverables
* Quick, document searching using language specific patterns ('class', 'extends', 'bool')
* Simple package installation steps for ease of use
* Room for expansion of language support/search features

## Milestones

**Starting Out**
[x] Build proficiency in Rustlang through reading documentation and doing some small projects.
[x] Working communication over JSON-RPC between the command line application and the language server.

**Single Language Support**
[x] Simple single document searches matching class names using the command line app.
[x] Expanded search capabilities within a single document using the application.
[x] Multi-Document search within a single directory on a single pattern (like class name) using the command line app.
[ ] Multi-Document search (single dir) on all viable patterns using the cli app.
[ ] Multi-Document search on all files within root and nested directories.

**Multi-Language Support**
[ ] Exand support to one additional language
[ ] Expand support to at least 5 languages total
[ ] Expand support to include up to 5 more languages
