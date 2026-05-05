<?php
	$jogadores = [
	    ["nome" => "Christopher", "pontos" => 1200],
	    ["nome" => "André", "pontos" => 676],
	    ["nome" => "Pedro", "pontos" => 2000],
	    ["nome" => "Paula", "pontos" => 1500]
	];

	$resultados = [
	    ["nome" => "Christopher", "resultado" => "ganhou", "valor" => 200],
	    ["nome" => "André", "resultado" => "ganhou", "valor" => 200],
	    ["nome" => "Pedro", "resultado" => "ganhou", "valor" => 200],
	    ["nome" => "Paula", "resultado" => "perdeu", "valor" => 400],
	];

	$mapa_jogadores = [];

	foreach ($jogadores as $j) {
		$mapa_jogadores[$j["nome"]] = $j;
	}

	foreach ($resultados as $evento => $resultado) {

    	if ($resultado["resultado"] == "ganhou") {
    		$mapa_jogadores[$resultado["nome"]]["pontos"] += $resultado["valor"];
    	} else {
    		$mapa_jogadores[$resultado["nome"]]["pontos"] -= $resultado["valor"];
    	}
    }

    usort($mapa_jogadores, function ($a, $b) {
	    return $b["pontos"] <=> $a["pontos"];
	});

	foreach ($mapa_jogadores as $posicao => $competidor) {
	    echo ($posicao + 1) . "º - " . $competidor["nome"] . " - " . $competidor["pontos"] . " pts\n";
	}
?>