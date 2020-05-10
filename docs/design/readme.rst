==========
Overview
==========

Here we talk about why I started Birlstone, especially, it appears to be
a practice of re-inventing wheels, given we know the existence of
Docutils_ and Pandoc_. We also need to talk about the design goals of
Birlstone, features and expected use scenarios.

Why re-inventing wheels?
==========================

Birlstone started from a needs of hobby project for me to balance my
drive-me-crazy daily work back to 2018, even if it never really started
until 2020.

I am always a big reStructuredText_ fan instead of Markdown_. Comparing
with Markdown_, reStructuredText_ supports some critical directives:
footnote links, cross reference, entity substitutions, etc. They make my
writing more like a book, well recognized and easy to modify. Markdown_,
on the other hand, is easy to write but also hard to avoid repeating
important tokens (e.g. URLs) everywhere. I prefer reStructuredText_
style even if it sometimes becomes too strict.

There are already several reStructuredText_ parsers: the standard
Docutils_, and popular Pandoc_. I used Docutils_ for years as my main
writing system for my blog, but it has pain points. One of them that
bugged me long is the newline-to-space conversion problem, which makes
me started thinking of creating a parser ourselves.

The details of newline-to-space problem, and why it bugs me
-------------------------------------------------------------

The story of newline-to-space conversion is visible in East Asia
languages. I writes my blogs in Chinese. When I write every newline in
reStructuredText_ is converted into a space, following Western languages.
This is completely fine for English, but makes generated Chinese
documents pretty ugly. I dig a bit and found in a mailing group or
somewhere (most closely
`here <https://sourceforge.net/p/docutils/mailman/message/26971957/>`_
but not original one), that Docutils_ author treated it as by design,
which should be responsibility of generated documentation. In my
context, it was HTML. However, by time we had this topic pops up,
there's no browser correctly handle indications like 'line-break'
correctly. Even by 2020 I can see only Firefox works as expected.
Though I do have background by escaping ``\`` newline, it makes text
format documentation cumbersome and pretty annoying.

I hold different opinion with author. I will say the assumption of
"newline should be treated as space" introduces an implicit assumption
from Western languages, while Docutils_ define other constructs in a way
that adaptable to almost all languages. The newline rule appears to be
the only exception. Passing the generated space does not make sense to
downstream documentation generator because they may lose context when
combining more formatting rules. Consider examples below.

::

   // '空格' and 'so' are two lines in RST, and usually should have a
   // space following a popular formatting convention of
   // Chinese/English mixed documentation in Internet.

   Here the new line should be a 空格
   so we can read.

   // '空格' and '因为‘ are two lines in RST but logically same line
   // in output format.

   Here the new line should NOT be a 空格
   因为 it's urly to read.

My understanding is this is documentation processor's responsibility to
provide full context so downstream generators can make correct choice.
And in practice, since typical formats like HTML and PDF does not handle
such convention perfectly, we should consider RST processor provide knobs
to handle it. It's unlikely to contribute to Docutils_ given it's already
considered by design, though.

And how about Pandoc?
-----------------------

Pandoc_ is popular too but it's indeed a format converter, while what
I'm looking for is a library that can be extended to scenarios like
hosting documentation site, just like how Docutils_ used to power
Pelican_. On the other hand, Pandoc_ is a bit overkill for me since it's
designed to support many input formats.

Pipeline of formatting
============================

I treat a conversion from reStructuredText_ to other formats as a
pipeline of three steps:

1. *Parser* to convert input document to intermediate format. The
   intermediate format should include both content and proper metadata.
2. *Formatter* tunes styles based on intermediate format. It uses
   customized rules with knowledge of how different languages are
   formatted. The output of **Formatter** supposes to be the
   intermediate format, with additional format-related metadata added.
3. *Generator* reads output of **Formatter** and finally generate the
   target format like HTML and PDF.

We can see step #2 and #3 may both include language specific rules,
which appears duplicated. However, by introducing #2 we have a chance to
take more control, especially when output format is in a lack of
required supports.

In Birlstone, *Parser* should be stable and only gets updated when we
extend reStructuredText_ constructs. *Generator* is also stable but
should be implemented by downstream implementation. *Formatter*, on the
other hand, is the most dynamic part that should be customized by users
in high frequency.

.. _reStructuredText: https://docutils.sourceforge.io/
.. _Markdown: https://daringfireball.net/projects/markdown/
.. _Docutils: https://docutils.sourceforge.io/
.. _Pandoc: https://pandoc.org/
.. _Pelican: https://blog.getpelican.com/
