//! Arte ASCII sombreado por densidad para el wyrm serpentino sin alas de WYRM Playground.
//!
//! Generado sobre lienzo vertical de alta resolución con compensación de relación de aspecto
//! para `JetBrains Mono` (`ancho : alto = 1 : 1.65`), perfil orientado hacia la izquierda.

/// Silueta principal sombreada (54×34) para el estado vacío de la Consola.
pub const WYRM_CONSOLE_ART: &str = r#"                                   XW           xX
                               X%#MW-    W##%##MW*
                            W%%Mx+  W#%@%#M*x+
                         X%%#x* W#@@%#Xxx-
                       M%%M*x#%@@#WX*+;     xWM##M#W
                     #%%WxW%@@#Wxx-   XW%%%%MWX++
                   M%%WW%@@%Mxx*: W%%@%MWx++
             *XWWW%%MM@@@#Wx*xM%%@%MW**-
   *     *XWMM########%#MMM#@@@%MW#Wx
  WMWXWWMMM### ◈ ######@@@@@@@%%@%%##%#WW*   WX
*XWMMMMMMMWWWXXXXXXXXX#%@@@%@@@@%%@@@%#%#WMMWM
-*XWMWXWWW+--      -xxWWX*x+***xxWXM#%%%%%#M#W+
   ##X  M        xXWMMWX*-*++++*+x*x*xxXM####MX*
      W  X#x +xXWWMMWWWWW+,,:-;-;--++x*xXXM#WMMXx-
    ;xXx*xXXxXXXxxx*+--+**       ::;;--**XxWMWWx*+
   +*xxx***+++-;                      :-*xWMMMWXxx;
     ;;;                                 xWM#%X**+:
                       Mx             XM#%%@@%XXXX***
             M         #MXXX#####%%%%%@%@@@@@%xWMWWWWX
            x#xXM#%#%@%@%%@@@%%@@@@@@@@@@@@#W*x*+;,
         X#%%%%@@%@@@@@@@@@@@@@@@@@####WX**+*+-++:
      W%@@@@@@@@@@@@%##MWWWWX***x*x***+*+**+++;:
  MM#@@@@@@@@#MWXx****x*x*+*+*+**++-++++---::,
   %@@@@%MX***+*+*+++++-+;----;;;;-:;:,,
  %@@@%x*+++-+-+;-;;:,:,:
 X%%@%*x++;-;:,
 W%@%W**+;
 XW##WXWXx                                         WW
  XWMMW#MMWW*                                     %x
  +xxXWWM##M%MWx       x                       M%W;
    +xxXXxW##%%#%##MXX XM      Xx          W%%#x;
      ;***x*xXX#%%@%%%%%%%###M#M#WW##%%@%@#Wx+:
        :-+-***x+**XWX####M@@@@@%###MWXx*--:
            ::;-;---+-+++*+**+*+*+-+-;;"#;

/// Versión compacta sombreada (39×20) con la misma proporción vertical para el Códice de Wyrm.
pub const WYRM_CODEX_ART: &str = r#"                        WMM+   WWM##W
                    M%#WxX#%%##WXx+;
                 #%#WMM@%MM*x-  WWM##W*
              M@#M%%%#WxxxW#@##WW**-
         xWWW##M@@#MWM#%%##XxM;
 WWXXWWMMM ◈ MMM#@@@@@%@%%#%##WW
xXWWXWWXxxx***xx#MMWXW###M@%%%%#MW*
  #W       +xWMMWX+*+*++**x*xXM%#MWx
   xW+XWxxxxxx*++x*,,;;;;-++*xxXMMMx*
  -+++--;;                 ,:-*XMMWXx**
              M XXXWWMM###%%@%@@%%XWWXx
    W  XW#%%%%%@%@@@%@@@@@@@@@@#Mx*x-,
   W#@@%@@@@@@@@%##MM#WWXXXW****+*-;:
  %@@@@#MWXX**x*x+****+*+*+*-+--;-:
 %@@#X**+*+++-+;-;--;;:;::,:,
W#@M**+;::,,
xMMMMMMMx                            WW
 +XWXW#%%%##WXx                   W#W-
   +**xxXW#%@%%%%%%########%%%@MMx+
     ;--++*+*+*xXXXWXWXWXWXX++-;"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wyrm_arts_have_single_eye_and_portrait_dimensions() {
        for (name, art, min_cols, max_cols, min_rows, max_rows) in [
            ("WYRM_CONSOLE_ART", WYRM_CONSOLE_ART, 52, 56, 30, 34),
            ("WYRM_CODEX_ART", WYRM_CODEX_ART, 38, 42, 18, 20),
        ] {
            assert_eq!(
                art.chars().filter(|&ch| ch == '◈').count(),
                1,
                "{name} debe contener exactamente un glifo de ojo '◈'"
            );
            let lines: Vec<&str> = art.lines().collect();
            assert!(
                (min_rows..=max_rows).contains(&lines.len()),
                "{name} tiene {} filas fuera del rango {min_rows}..={max_rows}",
                lines.len()
            );
            let max_width = lines
                .iter()
                .map(|line| line.chars().count())
                .max()
                .unwrap_or(0);
            assert!(
                (min_cols..=max_cols).contains(&max_width),
                "{name} tiene {max_width} cols fuera del rango {min_cols}..={max_cols}"
            );
            for line in lines {
                assert_eq!(
                    line,
                    line.trim_end(),
                    "{name} no debe tener espacios sobrantes al final de línea"
                );
            }
        }
    }
}
