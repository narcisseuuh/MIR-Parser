let () =
  let open Sys in
  let dir = "." in
  let files = readdir dir |> Array.to_list in
  let rs_files = List.filter (fun f -> Filename.check_suffix f ".rs") files in
  List.iter
    (fun file ->
      let mir = Rustc_parser.Rustc_ast.get_mir file in
      let pp_mir fmt bodies =
        List.iter (Rustc_parser.Rustc_pp.pp_body fmt) bodies
      in
      let out_file = Filename.chop_extension file ^ ".output" in
      let oc = open_out out_file in
      let fmt = Format.formatter_of_out_channel oc in
      Format.fprintf fmt "%a" pp_mir mir;
      close_out oc
    ) rs_files
