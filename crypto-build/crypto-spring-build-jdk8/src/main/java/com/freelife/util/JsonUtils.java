package com.freelife.util;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.core.StreamReadFeature;
import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.DeserializationFeature;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.PropertyNamingStrategy;
import com.fasterxml.jackson.databind.SerializationFeature;
import com.fasterxml.jackson.datatype.jdk8.StreamSerializer;
import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule;
import org.apache.commons.lang3.StringUtils;
import org.springframework.http.converter.json.Jackson2ObjectMapperBuilder;

import java.io.File;
import java.io.IOException;
import java.text.SimpleDateFormat;
import java.util.Objects;

import static com.fasterxml.jackson.core.JsonParser.Feature.INCLUDE_SOURCE_IN_LOCATION;

/**
 * Created by mskwon on 2023/12/16.
 */
public class JsonUtils {
    private final ObjectMapper mapper;

    private JsonUtils() {
        mapper = new ObjectMapper();
    }

    /**
     * Gets instance.
     *
     * @return the instance
     */
    private static JsonUtils getInstance() {
        return new JsonUtils();
    }

    public static ObjectMapper getObjectMapper() {
        // JavaTimeModule javaTimeModule = new JavaTimeModule();
        // javaTimeModule.addDeserializer(LocalDateTime.class, new LocalDateTimeDeserializer(DateTimeFormatter.ofPattern("yyyy-MM-dd")));
        // JsonFormat.Value format = JsonFormat.Value.forPattern("yyyy-MM-dd");
        // getInstance().mapper.configOverride(LocalDate.class).setFormat(format);
        return getInstance().mapper
                // .setDateFormat(new SimpleDateFormat("yyyy-MM-dd"))
                // .registerModule(javaTimeModule)
                .registerModule(new JavaTimeModule())
                .disable(SerializationFeature.WRITE_DATES_AS_TIMESTAMPS)
                .disable(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES)
                .disable(SerializationFeature.FAIL_ON_EMPTY_BEANS)
                .enable(JsonParser.Feature.ALLOW_COMMENTS);
    }

    /**
     * To ObjectMapper pretty json string
     * @param object
     * @return
     */
    public static String toMapper(Object object){
        if(Objects.isNull(object)) return null;
        try {
            return getObjectMapper().writeValueAsString(object);
        } catch (JsonProcessingException e) {
            throw new RuntimeException(e);
        }
    }

    /**
     * To ObjectMapper pretty json string
     * @param object
     * @return
     */
    public static String toMapperPretty(Object object){
        if(Objects.isNull(object)) return null;
        try {
            return getObjectMapper().writerWithDefaultPrettyPrinter().writeValueAsString(object);
        } catch (JsonProcessingException e) {
            throw new RuntimeException(e);
        }
    }

    /**
     * To ObjectMapper Mapping File to json string
     * @param file 변환할 오브젝트 파일
     * @param clz 변환할 오브젝트 클래스
     * @param <T>
     * @return
     */
    public static<T> T toMapperObject(File file, Class<T> clz) {
        if(file == null) return null;
        try {
            return getObjectMapper().readValue(file, clz);
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    /**
     * To ObjectMapper Mapping String to json string
     * @param obj JSON String
     * @param clz 변환할 오브젝트 클래스
     * @param <T>
     * @return
     */
    public static<T> T toMapperObject(String obj, Class<T> clz) {
        if(StringUtils.isEmpty(obj)) return null;
        try {
            return getObjectMapper().readValue(obj, clz);
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    /**
     * To ObjectMapper Mapping String to json string
     * @param obj JSON String
     * @param type 변환할 오브젝트 클래스
     * @param <T>
     * List 와 같은 형태는 TypeReference 형으로 처리
     * new TypeReference<List<Object>() {}
     * @return
     */
    public static<T> T toMapperObject(String obj, TypeReference<T> type) {
        if (StringUtils.isEmpty(obj)) return null;
        if(StringUtils.isEmpty(obj)) return (T) new TypeReference<T>() {};
        try {
            return getObjectMapper().readValue(obj, type);
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    public static <T> String toJsonString(T obj, boolean isPretty) {
        if(Objects.isNull(obj)) return null;
        Jackson2ObjectMapperBuilder b = new Jackson2ObjectMapperBuilder();
        b.propertyNamingStrategy(PropertyNamingStrategy.UPPER_CAMEL_CASE);
        SimpleDateFormat df = new SimpleDateFormat("yyyy-MM-dd");
        b.dateFormat(df);
        b.serializationInclusion(JsonInclude.Include.NON_NULL);

        ObjectMapper mapper = b.build();

        try {
            if (isPretty) {
                return mapper.writerWithDefaultPrettyPrinter().writeValueAsString(obj);
            } else {
                return mapper.writeValueAsString(obj);
            }
        } catch (JsonProcessingException e) {
            return e.getMessage();
        }
    }
}
