|MasterBranchCI|_ |MyDevBranchCI|_

===================
Birlstone project
===================

Birlstone project tries to implement a ReStructuredText parser, which
parses ReStructuredText as input and convert to HTML/HTML5.

Birlston is developed and tested under Mac and Microsoft Windows platform. I
don't own a Linux development box, but all unit tests are also covered on Linux
(thanks to ``Travis-CI`` and ``Bitbucket pipelines``).

Build from source code
===========================

Birlstone requires the following tools to build:

- **Rust compiler toolkit**. The latest code of Birlstone is built and
  tested with Rust 1.14.+ (though lower version might probably work as well).
  Please install latest Rust compiler toolkit following the steps from
  rust-lang_ web site.

- **Git**. To checkout latest source code.

When installing Rust and Git, please make sure the path to
``rust``, ``cargo`` and ``git`` are all added to ``PATH`` environment variable.
In Linux and Mac, this step is usually handled automatically by installer; In
Microsoft Windows, please make sure you have checked the corresponding checkbox
in installation widzard dialog to let installer do it for you.

When all tools are installed to development box, open a terminal emulator window
(for Linux and Mac) or CMD/PowerShell window (Microsoft Windows), and run the
command below.

::

  git clone https://github.com/fuzhouch/birlstone
  cd birlstone
  cargo build --release

The command will build Birlstone core library as a static library, which can be
used by developers to integrate it as dependencies of their own packages.

Birlstone also contains a command line tool, ``bls2html``, which is used to
demonstrate how to use Birlstone API. The tool is not built
by default, which avoids enforing uncessary dependencies for library developers.
To tell Birlstone build it explicitly, please run ``cargo build`` command with
parameter below:

::

  cargo build --release --features "cmdline"

The command will build both library (``libbirlstone.rlib``) and executable file
(``bls2html``) under ``target\release`` folder.

Branching story and Release
=============================

The main release and development places is in Github_. There are two
major branches existing by default:

- Master: The release branch. All release points will be tagged here.
- Fuzhouch: My own development branch.

Both branches are integrated with Travis-CI, so you can see badges in the
beginning of this documentation.

Meanwhile, a mirror repository from Bitbucket_ also hosts my
development branch. Unlike the Github release repository, the ``master`` branch
of Bitbucket_ repository is mapped to ``fuzhouch`` branch in Github_.
The mirror repository is useful for developers who want to consume the
development code, since Cargo does not allow us specify branch name when trying
to consume dependencies directly via Git path. (reference here_)

.. |MasterBranchCI| image:: https://travis-ci.org/fuzhouch/birlstone.svg?branch=master
.. _MasterBranchCI: http://github.com/fuzhouch/birlstone

.. |MyDevBranchCI| image:: https://travis-ci.org/fuzhouch/birlstone.svg?branch=fuzhouch
.. _MyDevBranchCI: https://github.com/fuzhouch/birlstone/tree/fuzhouch

.. _Bitbucket: https://bitbucket.org/fuzhouch/birlstone
.. _Github: http://github.com/fuzhouch/birlstone
.. _here: http://doc.crates.io/guide.html#cargotoml-vs-cargolock
.. _rust-lang: https://rust-lang.org
