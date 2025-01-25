package com.freelife;

import org.hibernate.validator.internal.util.privilegedactions.GetResource;
import org.junit.jupiter.api.Test;

import java.io.File;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.text.DateFormat;
import java.text.SimpleDateFormat;
import java.time.LocalDateTime;
import java.time.OffsetDateTime;
import java.time.ZoneId;
import java.time.format.DateTimeFormatter;
import java.util.*;

public class UtilTest {

    @Test
    void testFileCheck() {
        fileCheck();
    }


    private void fileCheck() {
        List<Path> basePaths  = new ArrayList<Path>();
        basePaths.add(Path.of("crypto","config.json").toAbsolutePath());
        basePaths.add(Path.of(File.pathSeparator,"var", "opt", "crypto","config", "config.json"));

        for (Path path : basePaths) {
            if (Files.exists(path)) {
                System.out.println("File exists: " + path);
            } else {
                System.out.println("File does not exist: " + path);
            }
        }

    }

    @Test
    void dateTest() {
        System.out.println(new SimpleDateFormat("yyyy-MM-dd HH:mm:ss:SSS").format(new Date()));;
        System.out.println(LocalDateTime.now().format(DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss:SSS")));
        System.out.println(LocalDateTime.now().format(DateTimeFormatter.ISO_DATE_TIME));
        System.out.println(LocalDateTime.now().format(DateTimeFormatter.BASIC_ISO_DATE));
        System.out.println(LocalDateTime.now().format(DateTimeFormatter.ISO_LOCAL_DATE_TIME));
        System.out.println(OffsetDateTime.now().format(DateTimeFormatter.ISO_OFFSET_DATE_TIME));
    }

}
