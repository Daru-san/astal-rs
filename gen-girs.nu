let paths = ($env.GIR_DIRS | split row --regex ":");

cd (git rev-parse --show-toplevel);

rm -rf girs
for dir in $paths {
    let files = ($dir | glob $"($dir)/*.gir");

    if (not ("girs" | path exists)) {
        mkdir "girs";
    }

    for file in $files {
        let out = cat $file | str replace --regex "VERSION\" c:identifier" "VERSION\" c:type" --all;
        let path = ($file | path basename);
        $out | save -f $"girs/($path)"
    }
}
