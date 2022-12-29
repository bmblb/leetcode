$lang = $args[0];
$demo = $args[1];

if ($args.Count -eq 1) {
    $demo = $lang;
    $lang = "all";
}

function Invoke-RunRust {
    $path = Resolve-Path -Path "rust\leet\target\release\leet.exe" -Relative;
    
    Write-Output("Rust");
    Invoke-Expression "$path $demo"
}

function Invoke-RunJS {
    $path = Resolve-Path -Path "node\\index_node.mjs" -Relative;

    Write-Output("Node");
    node --experimental-modules $path $demo

    $path = Resolve-Path -Path "node\\index_deno.js" -Relative;

    Write-Output("Deno");
    deno run --allow-hrtime $path $demo
}

if ($lang -eq "all") {
    Invoke-RunJS;
    Invoke-RunRust;
}
elseif ($lang -eq "rust") {
    Invoke-RunRust;
}
elseif ($lang -eq "node") {
    Invoke-RunJS;
}
