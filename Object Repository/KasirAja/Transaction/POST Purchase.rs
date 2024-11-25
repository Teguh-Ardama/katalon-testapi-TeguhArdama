<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST Purchase</name>
   <tag></tag>
   <elementGuidId>f8fbeb0c-ec0b-4903-bfe0-3646971ecd33</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;officeId\&quot;: \&quot;${officeId}\&quot;,\n    \&quot;date\&quot;: \&quot;${currentdate}\&quot;,\n    \&quot;invoice\&quot;: \&quot;INV/02/12/2023/001\&quot;,\n    \&quot;amount\&quot;: 14000,\n    \&quot;discount\&quot;: 0,\n    \&quot;description\&quot;: \&quot;test saja\&quot;,\n    \&quot;items\&quot; : [\n        {\n            \&quot;productId\&quot;: \&quot;${productId-1}\&quot;,\n            \&quot;quantity\&quot;: 4,\n            \&quot;cost\&quot;: 3500\n        }\n    ]\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/purchases</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>71767233-775d-4d3c-b0dd-df81dabd34c2</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.officeId</defaultValue>
      <description></description>
      <id>743a5899-e3e2-4e0e-8755-6b532dfec1d8</id>
      <masked>false</masked>
      <name>officeId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.currentdate</defaultValue>
      <description></description>
      <id>77af4e47-ae19-4f84-aa63-ceba7438eb5c</id>
      <masked>false</masked>
      <name>currentdate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.productId-1</defaultValue>
      <description></description>
      <id>978f34de-964c-41b8-82cb-c48c4ecf8010</id>
      <masked>false</masked>
      <name>productId-1</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
